require 'prime'
require_relative 'fnv'

# Based on the "Hash, displace, and compress" white paper.
class MinimalPerfectHash
  include FNV

  def self.generate!(keys)
    new(keys).tap(&:generate!)
  end

  attr_reader :salts, :slots

  # Supports:
  # * 32-bit unsigned ints
  # * 64-bit unsigned ints
  # * ASCII strings
  # * unsigned byte arrays
  def initialize(keys)
    @keys = keys.sort.to_set

    # We can use the fact that there's a high enough probability of collisions
    # to reduce the salts table a little bit, so that its size is always a prime number.
    # The prime number only helps reduce collisions when using different salt values.
    @salts_size = Prime.each(2*@keys.size).lazy.take_while { _1 <= @keys.size }.max

    # Automatically detect the types of the keys:
    case @keys.first
      when String
        def self.hash_key(salt, key) = fnv_hash_str(salt, key, @salts_size)
      when Integer
        if @keys.max <= 0xffff_ffff
          def self.hash_key(salt, key) = fnv_hash_u32(salt, key, @salts_size)
        else
          def self.hash_key(salt, key) = fnv_hash_u64(salt, key, @salts_size)
        end
      else
        def self.hash_key(salt, key) = fnv_hash_bytes(salt, key, @salts_size)
    end
  end

  # Returns:
  # * an array of salts
  #   * 0 is an empty slot
  #   * `+salt` must be hashed again with `hash_key(salt - 1)`
  #   * `-salt` are addressed directly with `-salt-1`
  #   * its size is == max prime <= `@keys.size`
  # * a Hash of `{ key => final slot }`
  #   * its size is == `@keys.size`
  def generate!
    return if @salts

    # create the buckets
    collisions, non_collisions = into_buckets()

    @salts = Array.new(@salts_size, 0)
    @slots = Array.new(@keys.size)

    collisions.each do |bucket|

      salt, slots = (1..100).each do |salt|
        slots = bucket.map { |key| hash_key(salt, key) }
        if slots.all? { |slot| @slots[slot].nil? } && slots.to_set.size == slots.size
          break [salt, slots]
        end
      end

      # If no unique unused slots found, the previous loop
      # returns the range `1..100` instead of `[salt, slots]`
      raise "Couldn't find unique slots for these keywords: #{bucket}" unless salt.is_a?(Integer)

      i = hash_key(0, bucket.first)
      # Add 1 to ensure it's not confused with an empty slot, which will contain 0
      @salts[i] = salt + 1
      slots.zip(bucket).each do |slot, key|
        @slots[slot] = key
      end
    end

    # single items
    if non_collisions.any?
      free_list = @slots.each_with_index.filter_map { |v, i| i if v.nil? }
      non_collisions.zip(free_list).each do |key, slot|
        i = hash_key(0, key)
        # Subtract 1 to ensure it's negative even if slot 0 was used
        @salts[i] = -slot-1
        @slots[slot] = key
      end
    end

    @slots = @slots.each_with_index
      .reject { |v, _| v.nil? }
      .to_h { |v, i| [v, i] }

    nil
  end

  def into_buckets
    buckets = @keys.group_by { |key| hash_key(0, key) }.values
    collisions, non_collisions = buckets.partition { |b| b.size > 1 }
    collisions.sort_by! { |b| -b.size }
    [collisions, non_collisions.flatten]
  end

end
