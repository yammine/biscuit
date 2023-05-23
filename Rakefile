# frozen_string_literal: true

require "bundler/gem_tasks"
require "rake/testtask"
require "rb_sys/extensiontask"

Rake::TestTask.new(:test) do |t|
  t.libs << "test"
  t.libs << "lib"
  t.test_files = FileList["test/**/test_*.rb"]
end

require "standard/rake"

require "rake/extensiontask"

task build: :compile

GEMSPEC = Gem::Specification.load("biscuit.gemspec") || abort("Could not load biscuit.gemspec")

RbSys::ExtensionTask.new("biscuit", GEMSPEC) do |ext|
  ext.lib_dir = "lib/biscuit"
end

task default: %i[compile test standard]
