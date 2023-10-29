INCLUDE "hardware.inc/hardware.inc"
INCLUDE "header.inc"


SECTION "Boot ROM", ROM0[$000]

EntryPoint:
    ld sp, hStackBottom

    xor a
    ld hl, $9FFF
.clearVRAM
    ld [hld], a
    bit 7, h
    jr nz, .clearVRAM

    ld hl, rNR52
    ld c, LOW(rNR11) ; CH1 length
    ; Enable APU
    ; This sets (roughly) all audio registers to 0
    ld a, $80
    ld [hld], a
    ; hl = rNR51
    ; Set CH1 duty cycle to 25%
    ldh [c], a
    inc c ; ld c, LOW(rNR11) ; CH1 envelope
    ld a, $F3 ; Initial volume 15, 3 decreasing sweep
    ldh [c], a
    ; Route all channels to left speaker, CH2 and CH1 to right speaker
    ld [hld], a
    ; hl = rNR50
    ; Set volume on both speakers to 7, disable VIN on both speakers
    ld a, $77
    ld [hl], a

    ld a, $FC
    ldh [rBGP], a

    ld hl, HeaderLogo
    push hl
    ld de, Logo
.checkLogo
    ld a, [de]
    inc de
    cp [hl]
    jr nz, Lockup
    inc hl
    ld a, l
    cp LOW(HeaderTitle)
    jr nz, .checkLogo
    ld b, HeaderChecksum - HeaderTitle
    ld a, b
.computeChecksum
    add a, [hl]
    inc hl
    dec b
    jr nz, .computeChecksum
    add a, [hl]
    jr nz, Lockup
    pop de ; ld de, HeaderLogo
    ld hl, vLogoTiles
.decompressLogo
    ld a, [de]
    call DecompressFirstNibble
    call DecompressSecondNibble
    inc de
    ld a, e
    cp LOW(HeaderTitle)
    jr nz, .decompressLogo

    ld a, $18

    ld hl, vMainTilemap + SCRN_VX_B * 9 + 15
.writeTilemapRow
    ld c, 12
.writeTilemapByte
    ld [hld], a
    dec a
    jr z, ScrollLogo
    dec c
    jr nz, .writeTilemapByte
    ; Go to previous row
    ld de, -(SCRN_VX_B - 12)
    add hl, de
    jr .writeTilemapRow


ScrollLogo:
    ; a = 0
    ld h, a ; ld h, 0
    ld a, $64
    ld d, a
    ldh [rSCY], a
    ld a, LCDCF_ON | LCDCF_BLK01 | LCDCF_BGON
    ldh [rLCDC], a
    inc b ; ld b, 1

    ; h = Number of times the logo was scrolled up
    ; d = How many frames before exiting the loop
    ; b = Whether to scroll the logo

.loop
    ld e, 2
    call DelayFrames
    ld c, LOW(rNR13) ; CH1 frequency low byte
    inc h
    ld a, h
    ld e, $83
    cp $62
    jr z, .playSound
    ld e, $C1
    cp $64
    jr nz, .dontPlaySound
.playSound
    ld a, e
    ldh [c], a
    inc c ; ld c, LOW(rNR14) ; CH1 frequency high byte
    ; Set frequency to $7XX and restart channel
    ld a, $87
    ldh [c], a
.dontPlaySound
    ldh a, [rSCY]
    sub b
    ldh [rSCY], a
    dec d
    jr nz, .loop

    dec b
    jr nz, Done
    ld d, $20
    jr .loop


Lockup:
    ld a, LCDCF_ON | LCDCF_BLK01 | LCDCF_BGON
    ldh [rLCDC], a
.loop
    ld e, 20
    call DelayFrames
    ldh a, [rBGP]
    xor a, $FF
    ldh [rBGP], a
    jr .loop


DecompressFirstNibble:
    ld c, a
DecompressSecondNibble:
    ld b, 8 / 2 ; Set all 8 bits of a, "consuming" 4 bits of c
.loop
    push bc
    rl c ; Extract MSB of c
    rla ; Into LSB of a
    pop bc
    rl c ; Extract that same bit
    rla ; So that bit is inserted twice in a (= horizontally doubled)
    dec b
    jr nz, .loop
    ld [hli], a
    inc hl ; Skip second plane
    ld [hli], a ; Also double vertically
    inc hl
    ret


DelayFrames:
    ld c, 12
.loop
    ldh a, [rLY]
    cp SCRN_Y
    jr nz, .loop
    dec c
    jr nz, .loop
    dec e
    jr nz, DelayFrames
    ret


; Each tile is encoded using 2 (!) bytes
; How to read: the logo is split into two halves (top and bottom), each half being encoded
;              separately. Each half must be read in columns.
;              So, the first byte is `db %XX.._XXX.`, then `db %XXX._XX.X`, matching the
;              `db $CE, $ED` found in many places. And so on! :)
MACRO logo_row_gfx
    ASSERT _NARG % 4 == 0
    PUSHO
    OPT b.X
    FOR N1, 1, _NARG / 4 + 1 ; N1, N2, N3, and N4 iterate through the 4 equally-sized rows
        DEF N2 = N1 + _NARG / 4
        DEF N3 = N2 + _NARG / 4
        DEF N4 = N3 + _NARG / 4
        db %\<N1>\<N2>, %\<N3>\<N4>
    ENDR
    POPO
ENDM

; Whitespace is not stripped after line continuations until RGBDS v0.6.0, so rows are not indented
    Logo:  logo_row_gfx \
XX.., .XX., XX.., ...., ...., ...., ...., ...., ...., ...X, X..., ...., \
XXX., .XX., XX.., ...., ..XX, ...., ...., ...., ...., ...X, X..., ...., \
XXX., .XX., ...., ...., .XXX, X..., ...., ...., ...., ...X, X..., ...., \
XX.X, .XX., XX.X, X.XX, ..XX, ..XX, XX.., XX.X, X..., XXXX, X..X, XXX.
           logo_row_gfx \
XX.X, .XX., XX.X, XX.X, X.XX, .XX., .XX., XXX., XX.X, X..X, X.XX, ..XX, \
XX.., XXX., XX.X, X..X, X.XX, .XXX, XXX., XX.., XX.X, X..X, X.XX, ..XX, \
XX.., XXX., XX.X, X..X, X.XX, .XX., ...., XX.., XX.X, X..X, X.XX, ..XX, \
XX.., .XX., XX.X, X..X, X.XX, ..XX, XXX., XX.., XX.., XXXX, X..X, XXX.


    ds 2
Done:
    inc a
    ldh [$FF50], a
    assert @ == $100 ; Execution now falls through to the cartridge's header


SECTION "VRAM tiles", VRAM[$8000],BANK[0]

vBlankTile:
    ds $10
vLogoTiles:
    ds $10 * (HeaderTitle - HeaderLogo) / 2
vRTile:
    ds $10

SECTION "VRAM tilemap", VRAM[$9800],BANK[0]

vMainTilemap:
    ds SCRN_VX_B * SCRN_VY_B


SECTION "HRAM", HRAM[$FFEE]

    ds $10
hStackBottom:
