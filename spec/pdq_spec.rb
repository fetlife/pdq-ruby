# frozen_string_literal: true

RSpec.describe Pdq do
  it "has a version number" do
    expect(Pdq::VERSION).not_to be nil
  end

  it "has a pdq_hash method" do
    expect(Pdq.respond_to?(:pdq_hash)).to be true
  end

  it "returns an array of two elements, representing the hash and quality" do
    output = Pdq.pdq_hash("spec/fixtures/test.jpeg")
    expect(output.size).to eq(2)

    hash, quality = output
    expect(hash).to eq("31ab07638e54b8fc49bb6633b1449dc44ebb63f3934d9c5c6cb2678f935dc000")
    expect(quality).to eq(1.0)
  end
end
