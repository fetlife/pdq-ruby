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

Wrapper of darwinium-com's [pdqhash](https://github.com/darwinium-com/pdqhash) rust implementation.

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

TODO: Write usage instructions here

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and the created tag, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/fetlife/pdq-ruby. This project is intended to be a safe, welcoming space for collaboration, and contributors are expected to adhere to the [code of conduct](https://github.com/fetlife/pdq-ruby/blob/main/CODE_OF_CONDUCT.md).

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).

## Code of Conduct

Everyone interacting in the pdq-ruby project's codebases, issue trackers, chat rooms and mailing lists is expected to follow the [code of conduct](https://github.com/fetlife/pdq-ruby/blob/main/CODE_OF_CONDUCT.md).
