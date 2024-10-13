
module FNV

  FNV_PRIME = 0x0100_0000_01b3
  FNV_MASK = (1 << 64) - 1

  def fnv_hash(d, key, table_size)
    d = FNV_PRIME if d == 0
    hash = key.each_byte.reduce(d) do |c, memo|
      ((memo ^ c) * FNV_PRIME) & FNV_MASK
    end
    hash % table_size
  end

end
