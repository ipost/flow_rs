require 'pry'

PLACEHOLDER = 'PLACEHOLDER'
template = <<-TEMPLATE
digraph L {
 #{PLACEHOLDER} 
}
TEMPLATE

input = ARGF.read

model = []

if_regex = /\A\s*if .+? {.+?}/m
statement_regex = /\A.+?;/m

while input.length > 0
  if input[if_regex]

  else

  end
end
