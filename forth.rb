DICT = {
  'test' => '+'
}
STACK = []

def run(cmd)
  if cmd == 'bye'
    exit(0)
  elsif cmd == '.'
    print STACK.pop
  elsif cmd == '.s'
    print "<#{STACK.size}> "
    STACK.each { print "#{_1} " }
  elsif cmd == '+'
    if STACK.size < 2
      print "stack underflow"
      return false
    end
    STACK.push(STACK.pop + STACK.pop)
  elsif DICT.keys.include?(cmd)
    return run(DICT[cmd])
  elsif cmd.start_with?(':')
    name, *body = parse(cmd[1...-1])
    DICT[name] = body.join(' ')
  else
    begin
      STACK.push Integer(cmd)
    rescue ArgumentError
      print "#{cmd} ?"
      return false
    end
  end
  true
end

def parse(line)
  cmds = line.split
  if cmds.include?(':')
    return false if cmds.count(':') > 1 || cmds.count(';') > 1
    s, e = cmds.index(':'), cmds.index(';')
    return false if s > e
    cmds = cmds[...s] + [cmds[s..e].join(' ')] + cmds[e+1..]
  end
  cmds
end

while line = gets
  print "\033[1A\033[#{line.size}C"
  cmds = parse(line)
  ok = true
  cmds.each do |cmd|
    ok = run(cmd)
    break if ok == false
  end
  print " ok" if ok
  puts
end

