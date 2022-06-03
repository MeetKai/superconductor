#version 450
/* Copyright (c) 2020-2022 Hans-Kristian Arntzen
 *
 * Permission is hereby granted, free of charge, to any person obtaining
 * a copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sublicense, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to
 * the following conditions:
 *
 * The above copyright notice and this permission notice shall be
 * included in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
 * EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
 * MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 * IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
 * CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
 * TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE
 * SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

#extension GL_EXT_samplerless_texture_functions : require

#include "bitextract.h"

layout(set = 0, binding = 0) uniform utexture2D uInput;

const bool SIGNED = false;

layout(push_constant) uniform Registers
{
    ivec2 resolution;
} registers;


const int weight_table3[8] = int[](0, 9, 18, 27, 37, 46, 55, 64);
const int weight_table4[16] = int[](0, 4, 9, 13, 17, 21, 26, 30, 34, 38, 43, 47, 51, 55, 60, 64);

#define P2(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p) \
    (((a) << 0) | ((b) << 1) | ((c) << 2) | ((d) << 3) | \
    ((e) << 4) | ((f) << 5) | ((g) << 6) | ((h) << 7) | \
    ((i) << 8) | ((j) << 9) | ((k) << 10) | ((l) << 11) | \
    ((m) << 12) | ((n) << 13) | ((o) << 14) | ((p) << 15))

const int partition_table2[32] = int[](
    P2(0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1),
    P2(0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1),
    P2(0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1),
    P2(0, 0, 0, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1),
    P2(0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 1),
    P2(0, 0, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1),
    P2(0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1),
    P2(0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 1, 1),

    P2(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1),
    P2(0, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
    P2(0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1),
    P2(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1),
    P2(0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
    P2(0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1),
    P2(0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1),
    P2(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1),

    P2(0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1, 1, 1, 1),
    P2(0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0),
    P2(0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 0),
    P2(0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0),
    P2(0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0),
    P2(0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0),
    P2(0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0),
    P2(0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 0, 1),

    P2(0, 0, 1, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0),
    P2(0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0),
    P2(0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 0),
    P2(0, 0, 1, 1, 0, 1, 1, 0, 0, 1, 1, 0, 1, 1, 0, 0),
    P2(0, 0, 0, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 0),
    P2(0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0),
    P2(0, 1, 1, 1, 0, 0, 0, 1, 1, 0, 0, 0, 1, 1, 1, 0),
    P2(0, 0, 1, 1, 1, 0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 0));

const int anchor_table2[32] = int[](
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 2, 8, 2, 2, 8, 8, 15,
    2, 8, 2, 2, 8, 8, 2, 2);

struct DecodedInterpolation
{
    ivec3 ep0, ep1;
    int weight;
};

ivec3 interpolate_endpoint(DecodedInterpolation interp)
{
    ivec3 rgb = ((64 - interp.weight) * interp.ep0 + interp.weight * interp.ep1 + 32) >> 6;
    return rgb;
}

ivec3 unquantize_endpoint(ivec3 ep, int bits)
{
    ivec3 unq;
    if (SIGNED)
    {
        ep = ivec3_bitfieldExtract(ep, 0, bits);
        if (bits < 16)
        {
            ivec3 sgn = 1 - ((ep >> 30) & 2);
            ivec3 abs_ep = abs(ep);
            unq = ((abs_ep << 15) + 0x4000) >> (bits - 1);
            unq = imix(unq, ivec3(0), equal(ep, ivec3(0)));
            unq = imix(unq, ivec3(0x7fff), greaterThanEqual(abs_ep, ivec3((1 << (bits - 1)) - 1)));
            unq *= sgn;
        }
        else
            unq = ep;
    }
    else
    {
        ep = ivec3(uvec3_bitfieldExtract(uvec3(ep), 0, bits));
        if (bits < 15)
        {
            unq = ((ep << 15) + 0x4000) >> (bits - 1);
            unq = imix(unq, ivec3(0), equal(ep, ivec3(0)));
            unq = imix(unq, ivec3(0xffff), equal(ep, ivec3((1 << bits) - 1)));
        }
        else
            unq = ep;
    }
    return unq;
}

DecodedInterpolation decode_bc6_mode0(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    int r0 = extract_bits(payload, 5, 10);
    int g0 = extract_bits(payload, 15, 10);
    int b0 = extract_bits(payload, 25, 10);
    ep0 = ivec3(r0, g0, b0);

    if (part != 0)
    {
        int r2 = extract_bits_sign(payload, 65, 5);
        int g2 = extract_bits(payload, 41, 4) | (extract_bits_sign(payload, 2, 1) << 4);
        int b2 = extract_bits(payload, 61, 4) | (extract_bits_sign(payload, 3, 1) << 4);

        int r3 = extract_bits_sign(payload, 71, 5);
        int g3 = extract_bits(payload, 51, 4) | (extract_bits_sign(payload, 40, 1) << 4);
        int b3 = extract_bits(payload, 50, 1) | (extract_bits(payload, 60, 1) << 1) | (extract_bits(payload, 70, 1) << 2) |
                (extract_bits(payload, 76, 1) << 3) | (extract_bits_sign(payload, 4, 1) << 4);

        ep1 = ivec3(r3, g3, b3) + ep0;
        ep0 += ivec3(r2, g2, b2);
    }
    else
    {
        int r1 = extract_bits_sign(payload, 35, 5);
        int g1 = extract_bits_sign(payload, 45, 5);
        int b1 = extract_bits_sign(payload, 55, 5);
        ep1 = ivec3(r1, g1, b1) + ep0;
    }

    ep0 = unquantize_endpoint(ep0, 10);
    ep1 = unquantize_endpoint(ep1, 10);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode1(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    int r0 = extract_bits(payload, 5, 7);
    int g0 = extract_bits(payload, 15, 7);
    int b0 = extract_bits(payload, 25, 7);
    ep0 = ivec3(r0, g0, b0);

    if (part != 0)
    {
        int r2 = extract_bits_sign(payload, 65, 6);
        int g2 = extract_bits(payload, 41, 4) | (extract_bits(payload, 24, 1) << 4) | (extract_bits_sign(payload, 2, 1) << 5);
        int b2 = extract_bits(payload, 61, 4) | (extract_bits(payload, 14, 1) << 4) | (extract_bits_sign(payload, 22, 1) << 5);

        int r3 = extract_bits_sign(payload, 71, 6);
        int g3 = extract_bits(payload, 51, 4) | (extract_bits_sign(payload, 3, 2) << 4);
        int b3 = extract_bits(payload, 12, 2) | (extract_bits(payload, 23, 1) << 2) | (extract_bits(payload, 32, 1) << 3) |
                (extract_bits(payload, 34, 1) << 4) | (extract_bits_sign(payload, 33, 1) << 5);

        ep1 = ivec3(r3, g3, b3) + ep0;
        ep0 += ivec3(r2, g2, b2);
    }
    else
    {
        int r1 = extract_bits_sign(payload, 35, 6);
        int g1 = extract_bits_sign(payload, 45, 6);
        int b1 = extract_bits_sign(payload, 55, 6);
        ep1 = ivec3(r1, g1, b1) + ep0;
    }

    ep0 = unquantize_endpoint(ep0, 7);
    ep1 = unquantize_endpoint(ep1, 7);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode2(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    int r0 = extract_bits(payload, 5, 10) | (extract_bits(payload, 40, 1) << 10);
    int g0 = extract_bits(payload, 15, 10) | (extract_bits(payload, 49, 1) << 10);
    int b0 = extract_bits(payload, 25, 10) | (extract_bits(payload, 59, 1) << 10);
    ep0 = ivec3(r0, g0, b0);

    if (part != 0)
    {
        int r2 = extract_bits_sign(payload, 65, 5);
        int g2 = extract_bits_sign(payload, 41, 4);
        int b2 = extract_bits_sign(payload, 61, 4);

        int r3 = extract_bits_sign(payload, 71, 5);
        int g3 = extract_bits_sign(payload, 51, 4);
        int b3 = extract_bits(payload, 50, 1) | (extract_bits(payload, 60, 1) << 1) |
                (extract_bits(payload, 70, 1) << 2) | (extract_bits_sign(payload, 76, 1) << 3);

        ep1 = ivec3(r3, g3, b3) + ep0;
        ep0 += ivec3(r2, g2, b2);
    }
    else
    {
        int r1 = extract_bits_sign(payload, 35, 5);
        int g1 = extract_bits_sign(payload, 45, 4);
        int b1 = extract_bits_sign(payload, 55, 4);
        ep1 = ivec3(r1, g1, b1) + ep0;
    }

    ep0 = unquantize_endpoint(ep0, 11);
    ep1 = unquantize_endpoint(ep1, 11);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode3(uvec4 payload, int linear_pixel)
{
    int r0 = extract_bits(payload, 5, 10);
    int g0 = extract_bits(payload, 15, 10);
    int b0 = extract_bits(payload, 25, 10);
    int r1 = extract_bits(payload, 35, 10);
    int g1 = extract_bits(payload, 45, 10);
    int b1 = extract_bits(payload, 55, 10);

    ivec3 ep0 = ivec3(r0, g0, b0);
    ivec3 ep1 = ivec3(r1, g1, b1);
    ep0 = unquantize_endpoint(ep0, 10);
    ep1 = unquantize_endpoint(ep1, 10);

    int index = extract_bits(
        payload,
        max(64 + linear_pixel * 4, 65),
        linear_pixel == 0 ? 3 : 4);

    int w = weight_table4[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode6(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    int r0 = extract_bits(payload, 5, 10) | (extract_bits(payload, 39, 1) << 10);
    int g0 = extract_bits(payload, 15, 10) | (extract_bits(payload, 50, 1) << 10);
    int b0 = extract_bits(payload, 25, 10) | (extract_bits(payload, 59, 1) << 10);
    ep0 = ivec3(r0, g0, b0);

    if (part != 0)
    {
        int r2 = extract_bits_sign(payload, 65, 4);
        int g2 = extract_bits(payload, 41, 4) | (extract_bits_sign(payload, 75, 1) << 4);
        int b2 = extract_bits_sign(payload, 61, 4);

        int r3 = extract_bits_sign(payload, 71, 4);
        int g3 = extract_bits(payload, 51, 4) | (extract_bits_sign(payload, 40, 1) << 4);
        int b3 = extract_bits(payload, 69, 1) | (extract_bits(payload, 60, 1) << 1) |
                (extract_bits(payload, 70, 1) << 2) | (extract_bits_sign(payload, 76, 1) << 3);

        ep1 = ivec3(r3, g3, b3) + ep0;
        ep0 += ivec3(r2, g2, b2);
    }
    else
    {
        int r1 = extract_bits_sign(payload, 35, 4);
        int g1 = extract_bits_sign(payload, 45, 5);
        int b1 = extract_bits_sign(payload, 55, 4);
        ep1 = ivec3(r1, g1, b1) + ep0;
    }

    ep0 = unquantize_endpoint(ep0, 11);
    ep1 = unquantize_endpoint(ep1, 11);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode7(uvec4 payload, int linear_pixel)
{
    int r0 = extract_bits(payload, 5, 10) | (extract_bits(payload, 44, 1) << 10);
    int g0 = extract_bits(payload, 15, 10) | (extract_bits(payload, 54, 1) << 10);
    int b0 = extract_bits(payload, 25, 10) | (extract_bits(payload, 64, 1) << 10);

    int r1 = extract_bits_sign(payload, 35, 9);
    int g1 = extract_bits_sign(payload, 45, 9);
    int b1 = extract_bits_sign(payload, 55, 9);

    r1 += r0;
    g1 += g0;
    b1 += b0;

    ivec3 ep0 = ivec3(r0, g0, b0);
    ivec3 ep1 = ivec3(r1, g1, b1);
    ep0 = unquantize_endpoint(ep0, 11);
    ep1 = unquantize_endpoint(ep1, 11);

    int index = extract_bits(
        payload,
        max(64 + linear_pixel * 4, 65),
        linear_pixel == 0 ? 3 : 4);

    int w = weight_table4[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode10(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    int r0 = extract_bits(payload, 5, 10) | (extract_bits(payload, 39, 1) << 10);
    int g0 = extract_bits(payload, 15, 10) | (extract_bits(payload, 49, 1) << 10);
    int b0 = extract_bits(payload, 25, 10) | (extract_bits(payload, 60, 1) << 10);
    ep0 = ivec3(r0, g0, b0);

    if (part != 0)
    {
        int r2 = extract_bits_sign(payload, 65, 4);
        int g2 = extract_bits_sign(payload, 41, 4);
        int b2 = extract_bits(payload, 61, 4) | (extract_bits_sign(payload, 40, 1) << 4);

        int r3 = extract_bits_sign(payload, 71, 4);
        int g3 = extract_bits_sign(payload, 51, 4);
        int b3 = extract_bits(payload, 50, 1) | (extract_bits(payload, 69, 2) << 1) |
                (extract_bits(payload, 76, 1) << 3) | (extract_bits_sign(payload, 75, 1) << 4);

        ep1 = ivec3(r3, g3, b3) + ep0;
        ep0 += ivec3(r2, g2, b2);
    }
    else
    {
        int r1 = extract_bits_sign(payload, 35, 4);
        int g1 = extract_bits_sign(payload, 45, 4);
        int b1 = extract_bits_sign(payload, 55, 5);
        ep1 = ivec3(r1, g1, b1) + ep0;
    }

    ep0 = unquantize_endpoint(ep0, 11);
    ep1 = unquantize_endpoint(ep1, 11);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode11(uvec4 payload, int linear_pixel)
{
    int r0 = extract_bits(payload, 5, 10) | (extract_bits_reverse(payload, 43, 2) << 10);
    int g0 = extract_bits(payload, 15, 10) | (extract_bits_reverse(payload, 53, 2) << 10);
    int b0 = extract_bits(payload, 25, 10) | (extract_bits_reverse(payload, 63, 2) << 10);

    int r1 = extract_bits_sign(payload, 35, 8);
    int g1 = extract_bits_sign(payload, 45, 8);
    int b1 = extract_bits_sign(payload, 55, 8);

    r1 += r0;
    g1 += g0;
    b1 += b0;

    ivec3 ep0 = ivec3(r0, g0, b0);
    ivec3 ep1 = ivec3(r1, g1, b1);
    ep0 = unquantize_endpoint(ep0, 12);
    ep1 = unquantize_endpoint(ep1, 12);

    int index = extract_bits(
        payload,
        max(64 + linear_pixel * 4, 65),
        linear_pixel == 0 ? 3 : 4);

    int w = weight_table4[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode14(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    int r0 = extract_bits(payload, 5, 9);
    int g0 = extract_bits(payload, 15, 9);
    int b0 = extract_bits(payload, 25, 9);
    ep0 = ivec3(r0, g0, b0);

    if (part != 0)
    {
        int r2 = extract_bits_sign(payload, 65, 5);
        int g2 = extract_bits(payload, 41, 4) | (extract_bits_sign(payload, 24, 1) << 4);
        int b2 = extract_bits(payload, 61, 4) | (extract_bits_sign(payload, 14, 1) << 4);

        int r3 = extract_bits_sign(payload, 71, 5);
        int g3 = extract_bits(payload, 51, 4) | (extract_bits_sign(payload, 40, 1) << 4);
        int b3 = extract_bits(payload, 50, 1) | (extract_bits(payload, 60, 1) << 1) |
                (extract_bits(payload, 70, 1) << 2) |
                (extract_bits(payload, 76, 1) << 3) | (extract_bits_sign(payload, 34, 1) << 4);

        ep1 = ivec3(r3, g3, b3) + ep0;
        ep0 += ivec3(r2, g2, b2);
    }
    else
    {
        int r1 = extract_bits_sign(payload, 35, 5);
        int g1 = extract_bits_sign(payload, 45, 5);
        int b1 = extract_bits_sign(payload, 55, 5);
        ep1 = ivec3(r1, g1, b1) + ep0;
    }

    ep0 = unquantize_endpoint(ep0, 9);
    ep1 = unquantize_endpoint(ep1, 9);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode15(uvec4 payload, int linear_pixel)
{
    int r0 = extract_bits(payload, 5, 10) | (extract_bits_reverse(payload, 39, 6) << 10);
    int g0 = extract_bits(payload, 15, 10) | (extract_bits_reverse(payload, 49, 6) << 10);
    int b0 = extract_bits(payload, 25, 10) | (extract_bits_reverse(payload, 59, 6) << 10);

    int r1 = extract_bits_sign(payload, 35, 4);
    int g1 = extract_bits_sign(payload, 45, 4);
    int b1 = extract_bits_sign(payload, 55, 4);

    r1 += r0;
    g1 += g0;
    b1 += b0;

    ivec3 ep0 = ivec3(r0, g0, b0);
    ivec3 ep1 = ivec3(r1, g1, b1);
    ep0 = unquantize_endpoint(ep0, 16);
    ep1 = unquantize_endpoint(ep1, 16);

    int index = extract_bits(
        payload,
        max(64 + linear_pixel * 4, 65),
        linear_pixel == 0 ? 3 : 4);

    int w = weight_table4[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode18(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    int r0 = extract_bits(payload, 5, 8);
    int g0 = extract_bits(payload, 15, 8);
    int b0 = extract_bits(payload, 25, 8);
    ep0 = ivec3(r0, g0, b0);

    if (part != 0)
    {
        int r2 = extract_bits_sign(payload, 65, 6);
        int g2 = extract_bits(payload, 41, 4) | (extract_bits_sign(payload, 24, 1) << 4);
        int b2 = extract_bits(payload, 61, 4) | (extract_bits_sign(payload, 14, 1) << 4);

        int r3 = extract_bits_sign(payload, 71, 6);
        int g3 = extract_bits(payload, 51, 4) | (extract_bits_sign(payload, 13, 1) << 4);
        int b3 = extract_bits(payload, 50, 1) | (extract_bits(payload, 60, 1) << 1) |
                (extract_bits(payload, 23, 1) << 2) | (extract_bits_sign(payload, 33, 2) << 3);

        ep1 = ivec3(r3, g3, b3) + ep0;
        ep0 += ivec3(r2, g2, b2);
    }
    else
    {
        int r1 = extract_bits_sign(payload, 35, 6);
        int g1 = extract_bits_sign(payload, 45, 5);
        int b1 = extract_bits_sign(payload, 55, 5);
        ep1 = ivec3(r1, g1, b1) + ep0;
    }

    ep0 = unquantize_endpoint(ep0, 8);
    ep1 = unquantize_endpoint(ep1, 8);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode22(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    int r0 = extract_bits(payload, 5, 8);
    int g0 = extract_bits(payload, 15, 8);
    int b0 = extract_bits(payload, 25, 8);
    ep0 = ivec3(r0, g0, b0);

    if (part != 0)
    {
        int r2 = extract_bits_sign(payload, 65, 5);
        int g2 = extract_bits(payload, 41, 4) | (extract_bits(payload, 24, 1) << 4) | (extract_bits_sign(payload, 23, 1) << 5);
        int b2 = extract_bits(payload, 61, 4) | (extract_bits_sign(payload, 14, 1) << 4);

        int r3 = extract_bits_sign(payload, 71, 5);
        int g3 = extract_bits(payload, 51, 4) | (extract_bits(payload, 40, 1) << 4) | (extract_bits_sign(payload, 33, 1) << 5);
        int b3 = extract_bits(payload, 13, 1) | (extract_bits(payload, 60, 1) << 1) |
                (extract_bits(payload, 70, 1) << 2) | (extract_bits(payload, 76, 1) << 3) |
                (extract_bits_sign(payload, 34, 1) << 4);

        ep1 = ivec3(r3, g3, b3) + ep0;
        ep0 += ivec3(r2, g2, b2);
    }
    else
    {
        int r1 = extract_bits_sign(payload, 35, 5);
        int g1 = extract_bits_sign(payload, 45, 6);
        int b1 = extract_bits_sign(payload, 55, 5);
        ep1 = ivec3(r1, g1, b1) + ep0;
    }

    ep0 = unquantize_endpoint(ep0, 8);
    ep1 = unquantize_endpoint(ep1, 8);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode26(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    int r0 = extract_bits(payload, 5, 8);
    int g0 = extract_bits(payload, 15, 8);
    int b0 = extract_bits(payload, 25, 8);
    ep0 = ivec3(r0, g0, b0);

    if (part != 0)
    {
        int r2 = extract_bits_sign(payload, 65, 5);
        int g2 = extract_bits(payload, 41, 4) | (extract_bits_sign(payload, 24, 1) << 4);
        int b2 = extract_bits(payload, 61, 4) | (extract_bits(payload, 14, 1) << 4) | (extract_bits_sign(payload, 23, 1) << 5);

        int r3 = extract_bits_sign(payload, 71, 5);
        int g3 = extract_bits(payload, 51, 4) | (extract_bits_sign(payload, 40, 1) << 4);
        int b3 = extract_bits(payload, 50, 1) | (extract_bits(payload, 13, 1) << 1) |
                (extract_bits(payload, 70, 1) << 2) | (extract_bits(payload, 76, 1) << 3) |
                (extract_bits(payload, 34, 1) << 4) | (extract_bits_sign(payload, 33, 1) << 5);

        ep1 = ivec3(r3, g3, b3) + ep0;
        ep0 += ivec3(r2, g2, b2);
    }
    else
    {
        int r1 = extract_bits_sign(payload, 35, 5);
        int g1 = extract_bits_sign(payload, 45, 5);
        int b1 = extract_bits_sign(payload, 55, 6);
        ep1 = ivec3(r1, g1, b1) + ep0;
    }

    ep0 = unquantize_endpoint(ep0, 8);
    ep1 = unquantize_endpoint(ep1, 8);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

DecodedInterpolation decode_bc6_mode30(uvec4 payload, int linear_pixel, int part, int anchor_pixel)
{
    ivec3 ep0, ep1;

    if (part != 0)
    {
        int r2 = extract_bits(payload, 65, 6);
        int g2 = extract_bits(payload, 41, 4) | (extract_bits(payload, 24, 1) << 4) | (extract_bits(payload, 21, 1) << 5);
        int b2 = extract_bits(payload, 61, 4) | (extract_bits(payload, 14, 1) << 4) | (extract_bits(payload, 22, 1) << 5);

        int r3 = extract_bits(payload, 71, 6);
        int g3 = extract_bits(payload, 51, 4) | (extract_bits(payload, 11, 1) << 4) | (extract_bits(payload, 31, 1) << 5);
        int b3 = extract_bits(payload, 12, 2) | (extract_bits(payload, 23, 1) << 2) |
            (extract_bits(payload, 32, 1) << 3) | (extract_bits(payload, 34, 1) << 4) | (extract_bits(payload, 33, 1) << 5);

        ep0 = ivec3(r2, g2, b2);
        ep1 = ivec3(r3, g3, b3);
    }
    else
    {
        int r0 = extract_bits(payload, 5, 6);
        int g0 = extract_bits(payload, 15, 6);
        int b0 = extract_bits(payload, 25, 6);

        int r1 = extract_bits(payload, 35, 6);
        int g1 = extract_bits(payload, 45, 6);
        int b1 = extract_bits(payload, 55, 6);

        ep0 = ivec3(r0, g0, b0);
        ep1 = ivec3(r1, g1, b1);
    }

    ep0 = unquantize_endpoint(ep0, 6);
    ep1 = unquantize_endpoint(ep1, 6);

    int index = extract_bits(
        payload,
        max(81 + linear_pixel * 3 - int(linear_pixel > anchor_pixel), 82),
        (linear_pixel == 0 || linear_pixel == anchor_pixel) ? 2 : 3);

    int w = weight_table3[index];
    return DecodedInterpolation(ep0, ep1, w);
}

layout(location = 0) out vec4 uOutput;

void main()
{
    ivec2 coord = ivec2(gl_FragCoord.xy);
    
    ivec2 tile_coord = coord >> 2;
    ivec2 pixel_coord = coord & 3;
    int linear_pixel = 4 * pixel_coord.y + pixel_coord.x;
    uvec4 payload = texelFetch(uInput, tile_coord, 0);

    DecodedInterpolation interp;

    int mode = extract_bits(payload, 0, 5);
    int part_index = extract_bits(payload, 77, 5);
    int part = (partition_table2[part_index] >> linear_pixel) & 1;
    int anchor_pixel = anchor_table2[part_index];

    if ((mode & 2) == 0)
    {
        if ((mode & 1) != 0)
            interp = decode_bc6_mode1(payload, linear_pixel, part, anchor_pixel);
        else
            interp = decode_bc6_mode0(payload, linear_pixel, part, anchor_pixel);
    }
    else
    {
        switch (mode)
        {
        case 2:
            interp = decode_bc6_mode2(payload, linear_pixel, part, anchor_pixel);
            break;
        case 3:
            interp = decode_bc6_mode3(payload, linear_pixel);
            break;
        case 6:
            interp = decode_bc6_mode6(payload, linear_pixel, part, anchor_pixel);
            break;
        case 7:
            interp = decode_bc6_mode7(payload, linear_pixel);
            break;
        case 10:
            interp = decode_bc6_mode10(payload, linear_pixel, part, anchor_pixel);
            break;
        case 11:
            interp = decode_bc6_mode11(payload, linear_pixel);
            break;
        case 14:
            interp = decode_bc6_mode14(payload, linear_pixel, part, anchor_pixel);
            break;
        case 15:
            interp = decode_bc6_mode15(payload, linear_pixel);
            break;
        case 18:
            interp = decode_bc6_mode18(payload, linear_pixel, part, anchor_pixel);
            break;
        case 22:
            interp = decode_bc6_mode22(payload, linear_pixel, part, anchor_pixel);
            break;
        case 26:
            interp = decode_bc6_mode26(payload, linear_pixel, part, anchor_pixel);
            break;
        case 30:
            interp = decode_bc6_mode30(payload, linear_pixel, part, anchor_pixel);
            break;
        default:
            interp = DecodedInterpolation(ivec3(0), ivec3(0), 0);
            break;
        }
    }

    ivec3 rgba_result = interpolate_endpoint(interp);

    // Squeeze range.
    if (SIGNED)
    {
        ivec3 neg_result = 0x8000 | (((-rgba_result) * 31) >> 5);
        ivec3 pos_result = (rgba_result * 31) >> 5;
        rgba_result = mix(pos_result, neg_result, lessThan(rgba_result, ivec3(0)));

        // Fixup for -0.0. Seems to not be emitted by hardware decoder.
        rgba_result = mix(rgba_result, ivec3(0), equal(rgba_result, ivec3(0x8000)));
    }
    else
    {
        rgba_result = (rgba_result * 31) >> 6;
    }

    uOutput = vec4(
        unpackHalf2x16(rgba_result.x).x,
        unpackHalf2x16(rgba_result.y).x,
        unpackHalf2x16(rgba_result.z).x,
        unpackHalf2x16(0x3c00).x
    );
}