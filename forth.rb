stack = []
while line = gets
  print "\033[1A\033[#{line.size}C"
  words = line.split
  ok = true
  words.each do |word|
    case word
    when 'bye'
      exit(0)
    when '.'
      print stack.pop
    when '.s'
      print "<#{stack.size}> "
      stack.each { print "#{_1} " }
    when '+'
      if stack.size < 2
        ok = false
        print "stack underflow"
        break
      end
      stack.push(stack.pop + stack.pop)
    else
      begin
        stack.push Integer(word)
      rescue ArgumentError
        print "#{word} ?"
        ok = false
        break
      end
    end
  end
  print " ok" if ok
  puts
end
