# frozen_string_literal: true

require "bundler/gem_tasks"
require "rb_sys/extensiontask"
require "rspec/core/rake_task"
require "standard/rake"

RSpec::Core::RakeTask.new(:spec)

GEMSPEC = Gem::Specification.load("pdq.gemspec")

task default: %i[compile spec standard]

# Aliases
task build: :compile
task test: :spec

# == "rake release" enhancements ==============================================

Rake::Task["release"].enhance do
  puts "Don't forget to publish the release on GitHub!"
  system "open https://github.com/fetlife/pdq-ruby/releases"
end

task :verify_gemspec_files do
  git_files = `git ls-files -z`.split("\x0")
  gemspec_files = GEMSPEC.files.sort
  ignored_by_git = gemspec_files - git_files
  next if ignored_by_git.empty?

  raise <<~ERROR
    The `spec.files` specified in pdq.gemspec include the following files
    that are being ignored by git. Did you forget to add them to the repo? If
    not, you may need to delete these files or modify the gemspec to ensure
    that they are not included in the gem by mistake:

    #{ignored_by_git.join("\n").gsub(/^/, "  ")}

  ERROR
end

Rake::Task[:build].enhance [:verify_gemspec_files]

# == "rake bump" tasks ========================================================

task bump: %w[bump:bundler bump:ruby bump:year]

namespace :bump do
  task :bundler do
    sh "bundle update --bundler"
  end

  task :ruby do
    replace_in_file "pdq.gemspec", /ruby_version = .*">= (.*)"/ => RubyVersions.lowest
    replace_in_file ".standard.yml", /ruby_version: (.*)/ => RubyVersions.lowest
    replace_in_file ".github/workflows/ci.yml", /ruby: (\[.+\])/ => RubyVersions.all.inspect
  end

  task :year do
    replace_in_file "LICENSE.txt", /\(c\) (\d+)/ => Date.today.year.to_s
  end
end

# == "rust build" enhancements ==============================================

Rake::ExtensionTask.new("pdq") do |ext|
  ext.ext_dir = "ext/pdq" # Source dir with extconf.rb / Cargo.toml
  ext.lib_dir = "lib/pdq" # Where compiled .bundle will go
end

# == Helpers ================================================================

require "date"
require "open-uri"
require "yaml"

def replace_in_file(path, replacements)
  contents = File.read(path)
  orig_contents = contents.dup
  replacements.each do |regexp, text|
    raise "Can't find #{regexp} in #{path}" unless regexp.match?(contents)

    contents.gsub!(regexp) do |match|
      match[regexp, 1] = text
      match
    end
  end
  File.write(path, contents) if contents != orig_contents
end

module RubyVersions
  class << self
    def lowest
      all.first
    end

    def all
      patches = versions.values_at(:stable, :security_maintenance).compact.flatten
      sorted_minor_versions = patches.map { |p| p[/\d+\.\d+/] }.sort_by(&:to_f)
      [*sorted_minor_versions, "head"]
    end

    private

    def versions
      @_versions ||= begin
        yaml = URI.open("https://raw.githubusercontent.com/ruby/www.ruby-lang.org/HEAD/_data/downloads.yml")
        YAML.safe_load(yaml, symbolize_names: true)
      end
    end
  end
end
