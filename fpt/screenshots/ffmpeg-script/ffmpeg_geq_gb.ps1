$out = 'why_do_i_do_this.mp4'

$filter = @'
    geq=r='if(lt(p(X,Y),191),if(lt(p(X,Y),128),if(lt(p(X,Y),64),   0,  46), 140), 160)'
       :g='if(lt(p(X,Y),191),if(lt(p(X,Y),128),if(lt(p(X,Y),64),  63, 115), 191), 207)'
       :b='if(lt(p(X,Y),191),if(lt(p(X,Y),128),if(lt(p(X,Y),64),   0,  32),  10),  10)',
    scale=iw*5:ih*5:flags=neighbor
'@

if (Test-Path $out) { del $out }

ffmpeg -hide_banner -y `
    -framerate 120 `
    -i '../test_one_tile_to_vram-ly_%5d.pgm' `
    -vf "$filter" `
    -c:v libx264 `
    $out

if (Test-Path $out) { start $out }
