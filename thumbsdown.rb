#!/usr/bin/env ruby

# check for stuff we need to make this work:
# - imagemagick
# - mplayer

used_binaries = [ 'convert','mogrify','mplayer' ] 

used_binaries.each do |file|
  if `which #{file}` == ""
    puts "Error: missing #{file}"
    exit
  end
end

require 'optparse'
require 'fileutils'
require 'pp'

@start = 1
@thumbs = 20
@columns = 5
@output = "thumbs.png"
@temp = "/tmp"
@width = 320

ARGV.options do |opts|
  opts.banner = "\nUsage: #{$0} [OPTIONS]\n\nCreate a multi-thumbnail image of a video file\n\n"

  opts.on("-h", "--help", "show this message") {
    puts opts
    exit
  }
  opts.on("-v", "--[no-]verbose=[FLAG]", TrueClass, "run verbosly") {
    |@verbose|   # sets @verbose to true or false
  }
  opts.on("-o", "--output=STRING", String, "Output filename (default out.(png|jpeg))") {
    |@output|   
  }
  opts.on("-c", "--columns=INT", Integer, "Number of thumbnail columns (default: 5)") {
    |@columns|   
  }
  opts.on("-s", "--start=INT", Integer, "Start point for making thumbnails in seconds (default: 1)") {
    |@start|   
  }
  opts.on("-T", "--temp=STRING", String, "Specify a temporary directory (default /tmp)") {
    |@temp|   
  }
  opts.on("-t", "--thumbs=INT", Integer, "Number of thumbs to generate - the length of the video is divided equally by this (default: 20)") {
    |@thumbs|   
  }
  opts.on("-w", "--width=INT", Integer, "Width of a single thumbnail - Height calculated automaticly (default: 320px)") {
    |@width|   
  }
  opts.on("-f", "--[no-]force=[FLAG]", TrueClass, "force mode - override output") {
    |@force|   # sets @verbose to true or false
  }
  opts.parse!
end

if ARGV.length < 1
  #       puts "\nUsage: #{$0} [OPTIONS]\n\nCreate a multi-thumbnail image of a video file\n\n"
  puts ARGV.options 
  exit
end

identify_args = '-frames 1 -nosound -vo null -identify '
identify_cmd = "mplayer -nolirc " + identify_args + "\"" + ARGV[0] + "\""
thumb_args = '-osdlevel 2 -vo png -nosound -frames 1 -vf expand=-1:-1:-1:-1:1 '
thumb_cmd = "mplayer -nolirc " + thumb_args + "\"" + ARGV[0] + "\""

if ! File.exists?(ARGV[0])
  puts "Input video file does not exits"
  exit
end

if File.exists?(@output) && ! defined? @force 
  puts "Output file already exists: use -f to override"
  exit
end

if File.exists?(@output) && defined? @force 
  if defined? @verbose
    puts "Output file already exists: deleting"
  end
  File.unlink(@output)
end

if ! File.directory?(@temp) 
  puts "Temp directory does not exists"
  exit 
end

if ! File.writable?(@temp) 
  puts "Temp directory is not writable"
  exit 
end

if defined? @verbose
  puts "Temp directory: #{@temp} - exists and is writable"
end


id = Hash.new

`#{identify_cmd}`.grep(/^ID/).each do |line|
  key = line.chomp.split("=")[0]
  value = line.chomp.split("=")[1]
  id[key] = value
end

step = ( id['ID_LENGTH'].to_i / @thumbs ).to_i

for i in 1..@thumbs
  thumb_time = i * step
  break if thumb_time > id['ID_LENGTH'].to_i
  `#{thumb_cmd} -ss #{thumb_time}`
  index = sprintf("%.8d",i)
  #`mv 00000001.png screen-#{index}.png` 
  if defined? @verbose
    tt = sprintf("%.8d", thumb_time)
    puts "Dumping [Time: #{tt}] #{@temp}/screen-#{index}.png"
  end
  FileUtils.move "00000001.png", "#{@temp}/screen-#{index}.png" 
  `mogrify -border 10 -sample #{@width}x #{@temp}/screen-#{index}.png`
  #      puts "#{thumb_cmd} -ss #{thumb_time}"
end

header = File.basename(id['ID_FILENAME']) + "\nvcodec: " + id['ID_VIDEO_FORMAT'] + ", fps: " + id['ID_VIDEO_FPS'] + ", resolution: " + id['ID_VIDEO_WIDTH'] + "x" + id['ID_VIDEO_HEIGHT'] + "\n"

`echo "#{header}" | convert -background  none -pointsize 18 text:- -trim +repage -flatten #{@temp}/header.png`

thumbs = Dir.new(@temp).grep(/screen/).sort

len = thumbs.length

col = 0
row = String.new
cmd = "convert"

len.times do |index|
  row = row + " " + @temp + "/" + thumbs[index]
  col += 1
  if col == @columns || index + 1 == len
    row = row + " +append"
    col = 0
    cmd += " \\(" + row + " \\)"
    row = String.new
  end
end

cmd += " -append #{@temp}/tmp.png"

if defined? @verbose
  puts "Joining thumbs to: #{@output}"
  puts "Executing: " + cmd
end

`#{cmd}`

Dir.new(@temp).grep(/screen-.*png/).each do |file|
  if defined? @verbose
    puts "Deleting #{@temp}/#{file}"
  end
  File.unlink("#{@temp}/#{file}")
end

`convert -gravity center -background white #{@temp}/header.png #{@temp}/tmp.png -append #{@output}`

if defined? @verbose
  puts "Deleting #{@temp}/tmp.png"
end
File.unlink("#{@temp}/tmp.png")
if defined? @verbose
  puts "Deleting #{@temp}/header.png"
end
File.unlink("#{@temp}/header.png")

if defined? @verbose
  puts "DONE."
end
