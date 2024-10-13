require_relative 'fnv'

class MinimalPerfectHash
  include FNV

  def self.generate!(keys)
    new(keys).generate!
  end

  def initialize(keys)
    @keys = keys.sort.to_set
  end

  # Returns:
  # * an array of intermediate hashes
  #   * 0 is an empty slot
  #   * `-d` must be hashed again with `fnv_hash(d - 1)`
  #   * `+d` negative integers are addressed directly with `-d-1`
  # * a Hash of `{ key => final slot }`
  def generate!
    return [@intermediate, @slots] if @intermediate

    # create the buckets
    collisions, non_collisions = into_buckets()

    table_size = @keys.size
    @intermediate = Array.new(table_size, 0)
    @slots = Array.new(table_size)

    collisions.each do |bucket|

      d, slots = (1..100).each do |d|
        slots = bucket.map { |key| fnv_hash(d, key, table_size) }
        if slots.all? { |slot| @slots[slot].nil? } && slots.to_set.size == slots.size
          break [d, slots]
        end
      end

      # if no unique unused slots found, the previous loop returns the range `1..100` instead of `[d, slots]`
      raise "Couldn't find unique slots for these keywords: #{bucket}" unless d.is_a?(Integer)

      i = fnv_hash(0, bucket.first, table_size)
      # Add 1 to ensure it's not confused with an empty slot, which will contain 0
      @intermediate[i] = d + 1
      slots.zip(bucket).each do |slot, key|
        @slots[slot] = key
      end
    end

    # single items
    if non_collisions.any?
      free_list = @slots.each_with_index.filter_map { |v, i| i if v.nil? }
      non_collisions.zip(free_list).each do |key, slot|
        i = fnv_hash(0, key, table_size)
        # Subtract 1 to ensure it's negative even if slot 0 was used
        @intermediate[i] = -slot-1
        @slots[slot] = key
      end
    end

    @slots = @slots.each_with_index.to_h { |v, i| [v, i] }

    [@intermediate, @slots]
  end

  def into_buckets
    table_size = @keys.size
    buckets = @keys.group_by { |key| fnv_hash(0, key, table_size) }.values
    buckets.sort_by! { |b| -b.size }
    collisions, non_collisions = buckets.partition { |b| b.size > 1 }
    [collisions, non_collisions.flatten]
  end

end