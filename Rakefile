# frozen_string_literal: true

require "bundler/gem_tasks"
require "rspec/core/rake_task"

RSpec::Core::RakeTask.new(:spec)

require "rubocop/rake_task"

RuboCop::RakeTask.new

require "rb_sys/extensiontask"

task build: :compile

RbSys::ExtensionTask.new("filer") do |ext|
  ext.lib_dir = "lib/filer"
end

task default: %i[compile spec rubocop]
