# frozen_string_literal: true

require "test_helper"

class TestKeyPair < Minitest::Test
  def test_from
    kp = Biscuit::KeyPair.new
    from = Biscuit::KeyPair.from(kp.private)

    assert kp.public.to_hex == from.public.to_hex
    assert kp.private.to_hex == from.private.to_hex
  end
end
