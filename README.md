# Pdq-ruby

[![Gem Version](https://img.shields.io/gem/v/pdq-ruby)](https://rubygems.org/gems/pdq-ruby)
[![Gem Downloads](https://img.shields.io/gem/dt/pdq-ruby)](https://www.ruby-toolbox.com/projects/pdq-ruby)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/fetlife/pdq-ruby/ci.yml)](https://github.com/Pezmc/pdq-ruby/actions/workflows/ci.yml)
[![Code Climate maintainability](https://img.shields.io/codeclimate/maintainability/Pezmc/pdq-ruby)](https://codeclimate.com/github/Pezmc/pdq-ruby)

## Description

pdq-ruby is a Ruby gem that provides a convenient wrapper around the PDQ (Photo DNA Quick) hash algorithm implementation. PDQ is a perceptual image hashing algorithm developed by Facebook (now Meta) that generates compact binary fingerprints of images. These fingerprints can be used to:

- Detect duplicate or near-duplicate images
- Identify similar images even after modifications
- Perform efficient image similarity searches
- Support content moderation and copyright protection

The gem provides a Ruby-friendly interface to the Rust-based PDQ implementation, making it easy to:

- Generate PDQ hashes from images
- Compare images for similarity
- Handle various image formats
- Process images efficiently with native performance

This gem is a wrapper of darwinium-com's [pdqhash](https://github.com/darwinium-com/pdqhash) Rust implementation.

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'pdq', git: 'https://github.com/fetlife/pdq-ruby.git'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install pdq --source https://github.com/fetlife/pdq-ruby.git
```

## Usage

### Basic Usage

The gem provides a simple interface to generate PDQ hashes from images:

```ruby
require 'pdq'

# Generate a PDQ hash from an image file
hash, quality = Pdq.pdq_hash("path/to/image.jpg")

# The hash is returned as a hexadecimal string
puts "PDQ Hash: #{hash}"
# => PDQ Hash: 31ab07638e54b8fc49bb6633b1449dc44ebb63f3934d9c5c6cb2678f935dc000

# The quality score indicates the quality of hash (0 - 1.0)
puts "Quality: #{quality}"
# => Quality: 1.0
```

### Understanding the Output

The `pdq_hash` method returns an array containing two elements:
1. The PDQ hash as a hexadecimal string (256 bits, represented as 64 hexadecimal characters)
2. A quality score between 0.0 and 1.0 indicating the reliability of the hash
   - 1.0 indicates a high-quality hash
   - Lower scores suggest potential issues with the image or hash generation

### Supported Image Formats

Supports any format handled by [image::ImageReader](https://crates.io/crates/image).

## Development

### Setup

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests.
You can also run `bin/console` for an interactive prompt that will allow you to experiment.

### Installation

To install this gem onto your local machine, run `bundle exec rake install`.

### Releasing a New Version

To release a new version:
1. Update the version number in `version.rb`
2. Run `bundle exec rake release`

This will:
- Create a git tag for the version
- Push git commits and the created tag
- Push the `.gem` file to [rubygems.org](https://rubygems.org)

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/fetlife/pdq-ruby. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/fetlife/pdq-ruby/blob/main/CODE_OF_CONDUCT.md).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the pdq-ruby project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/fetlife/pdq-ruby/blob/main/CODE_OF_CONDUCT.md).
