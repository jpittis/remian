require 'bundler/setup'
require 'helix_runtime/build_task'
require "rake/testtask"

HelixRuntime::BuildTask.new

Rake::TestTask.new do |t|
  t.libs << 'lib'
  t.libs << 'test'
  t.pattern = 'test/**/*_test.rb'
end
