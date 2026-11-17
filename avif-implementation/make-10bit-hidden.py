#!/usr/bin/env python3
"""Generate a 10-bit AVIF whose content is invisible in 8-bit rendering.

Background luma = 512, hidden text luma = 515 (out of 1024).
In 8-bit both map to 255 * v / 1023 -> round(127.687) = 128 and
round(128.435) = 128, i.e. identical. Only a 10-bit pipeline can show it.
"""
import struct
from PIL import Image, ImageDraw, ImageFont

W, H = 640, 320
BG, FG = 512, 515
FONT = "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf"

# Rasterize the hidden message as a mask
mask = Image.new("L", (W, H), 0)
d = ImageDraw.Draw(mask)
font = ImageFont.truetype(FONT, 96)
text = "10-BIT"
bbox = d.textbbox((0, 0), text, font=font)
tw, th = bbox[2] - bbox[0], bbox[3] - bbox[1]
d.text(((W - tw) // 2 - bbox[0], (H - th) // 2 - bbox[1]), text,
       fill=255, font=font)
mask_px = mask.getdata()

# Build Y plane; U and V stay neutral (512, full range)
y = bytearray(W * H * 2)
i = 0
for m in mask_px:
    v = FG if m else BG
    y[i] = v & 0xFF
    y[i + 1] = v >> 8
    i += 2
u = bytearray(struct.pack("<H", 512) * (W * H))
v = bytes(u)

with open("hidden-10bit.yuv", "wb") as f:
    f.write(y)
    f.write(u)
    f.write(v)
print("wrote hidden-10bit.yuv")
