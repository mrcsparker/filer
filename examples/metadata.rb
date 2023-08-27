# bundle exec ruby -rfiler examples/metadata.rb

require 'filer'

metadata = Filer::Metadata.new("README.md")
p metadata.is_file
p metadata.len

metadata = Filer::metadata("README.md")
p metadata.is_file
p metadata.len

# Filer::write("/tmp/foo.txt", "Hello World!")
