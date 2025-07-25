# frozen_string_literal: true

require_relative "lib/pdq/version"

Gem::Specification.new do |spec|
  spec.name = "pdq"
  spec.version = Pdq::VERSION
  spec.authors = ["Pez Cuckow"]
  spec.email = ["email@pezcuckow.com"]

  spec.summary = "A Ruby gem for PDQ functionality"
  spec.description = "A Ruby gem that provides PDQ functionality"
  spec.homepage = "https://github.com/fetlife/pdq-ruby"

  spec.license = "MIT"
  spec.required_ruby_version = ">= 3.2"
  spec.required_rubygems_version = ">= 3.3.11"

  spec.metadata["allowed_push_host"] = "https://rubygems.org"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/fetlife/pdq-ruby"
  spec.metadata["changelog_uri"] = "https://github.com/fetlife/pdq-ruby/releases"

  # Specify which files should be added to the gem when it is released.
  # The `git ls-files -z` loads the files in the RubyGem that have been added into git.
  gemspec = File.basename(__FILE__)
  spec.files = IO.popen(%w[git ls-files -z], chdir: __dir__, err: IO::NULL) do |ls|
    ls.readlines("\x0", chomp: true).reject do |f|
      (f == gemspec) ||
        f.start_with?(*%w[bin/ test/ spec/ features/ .git .github appveyor Gemfile])
    end
  end
  spec.bindir = "exe"
  spec.executables = spec.files.grep(%r{\Aexe/}) { |f| File.basename(f) }
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/pdq/extconf.rb"]

  # Uncomment to register a new dependency of your gem
  # spec.add_dependency "example-gem", "~> 1.0"
  spec.add_dependency "rb_sys", "~> 0.9.91"

  # For more information and examples about making a new gem, check out our
  # guide at: https://bundler.io/guides/creating_gem.html
end
