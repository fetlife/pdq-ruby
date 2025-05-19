# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "standard/rake"

require "rb_sys/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("pdq.gemspec")

RbSys::ExtensionTask.new("pdq", GEMSPEC) do |ext|
  ext.lib_dir = "lib/pdq"
end

task default: %i[compile spec standard]
