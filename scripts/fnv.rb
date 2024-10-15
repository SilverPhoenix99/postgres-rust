
module FNV

  FNV_PRIME = 0x0100_0193
  FNV_OFFSET_BASIS = 0x811c_9dc5
  FNV_MASK = (1 << 32) - 1

  def fnv_hash_str(salt, key, table_size)
    fnv_hash_bytes(salt, key.bytes, table_size)
  end

  def fnv_hash_u32(salt, key, table_size)
    bytes = (0..24).step(8).map { |i| (key >> i) & 0xff }
    fnv_hash_bytes(salt, bytes, table_size)
  end

  def fnv_hash_u64(salt, key, table_size)
    bytes = (0..56).step(8).map { |i| (key >> i) & 0xff }
    fnv_hash_bytes(salt, bytes, table_size)
  end

  # Modified version of FNV-1a, with an extra salt mixed in
  def fnv_hash_bytes(salt, key, table_size)
    hash = key.reduce(FNV_OFFSET_BASIS) do |acc, b|
      ((b ^ (acc + salt)) * FNV_PRIME) & FNV_MASK
    end

    # xor-shift excess bits
    nbits = table_size.bit_length
    n = 16
    mask = 0xffff
    while n > nbits
      hash = (hash >> n) ^ (hash & mask)
      n >>= 1
      mask >>= n
    end

    # lazy mod
    hash % table_size
  end

  extend FNV
end
