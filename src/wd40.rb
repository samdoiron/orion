#!/usr/bin/env ruby

require 'colorize'
require 'pry'

def is_beginning_of_error(line)
  !!line.match(/[\w\/]\.rs:\d+:\d+/)
end

def first_match(str, regex)
  (str.match(regex) || [])[1]
end

def parse_line(error_line)
  file_path, from_line, from_column, to_line, to_column = error_line.split(':')
  to_column = to_column.split(' ').first if to_column

  error_type = error_line.scan(/[a-z]+:/)[1]
  error_without_colon = error_type ? error_type[0..-2] : nil
  message = nil

  if error_without_colon == 'warning'
    message = first_match(error_line, /warning:(.+?)#/).strip
  end
  {
    :file_path   => file_path,
    :from_line   => from_line,
    :from_column => from_column,
    :to_line     => to_line.strip,
    :to_column   => to_column,
    :type        => error_without_colon,
    :message     => message
  }
end

def is_underline(line)
  line.include?("^~")
end

def is_header(line)
  line.include?("Compiling")
end

def parse(error_string)
  error_lines = error_string.split("\n")
  grouped_errors = []
  current_error = ""

  error_lines.each do |line|
    if is_beginning_of_error(line)
      grouped_errors.push(current_error) unless current_error.empty?
      current_error = line
    elsif !is_underline(line) && !is_header(line)
      current_error += "\n" + line
    end
  end

  grouped_errors.map(&:strip).map(&method(:parse_line))
end

def color_for(error_type)
  case error_type
  when "error"
    :red
  when 'warning'
    :yellow
  when "help"
    :blue
  else
    :green
  end
end

def show_parsed_error(parsed)
  puts [
    parsed[:type].colorize(color_for(parsed[:type])),
    " ",
    parsed[:file_path],
    " ",
    parsed[:from_line].colorize(:yellow),
    parsed[:message] ? "\n\t" : "",
    parsed[:message],
    "\n\n"
  ].join("")
end

def p_array(array)
  array.each do |item|
    puts item
    puts
  end
end

read = STDIN.read
parsed = parse(read)
parsed.each { |x| show_parsed_error(x) }
