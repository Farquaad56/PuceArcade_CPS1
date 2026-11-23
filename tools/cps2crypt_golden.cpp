// Oracle du déchiffrement CPS2 : compile ref/mame-cps2/cps2crypt.cpp (MAME 0.287) tel quel et produit
// tests-data/cps2crypt/vectors.txt (ROM et clés synthétiques, aucune donnée commerciale). spec: docs/spec/05 §Chiffrement
// g++ -std=c++17 -O2 -I tools/cps2crypt_shim -I ref/mame-cps2 tools/cps2crypt_golden.cpp ref/mame-cps2/cps2crypt.cpp -o target/cps2crypt_golden
#include "emu.h"
#include "cps2crypt.h"
#include <vector>

// init_cps2crypt (ref/mame-cps2/cps2.cpp l.10955-10992) : décodage du fichier .key (20 octets)
static void decode_key(const uint8_t *k, uint32_t key[2], uint32_t &lower, uint32_t &upper)
{
	unsigned short decoded[10] = { 0 };
	for (int b = 0; b < 10 * 16; b++)
	{
		int bit = (317 - b) % 160;
		if ((k[bit / 8] >> ((bit ^ 7) % 8)) & 1)
			decoded[b / 16] |= (0x8000 >> (b % 16));
	}
	key[0] = ((uint32_t)decoded[0] << 16) | decoded[1];
	key[1] = ((uint32_t)decoded[2] << 16) | decoded[3];
	if (decoded[9] == 0xffff) { upper = 0xffffff; lower = 0xff0000; }
	else { upper = (((~decoded[9] & 0x3ff) << 14) | 0x3fff) + 1; lower = 0; }
}

// inverse de decode_key : place les 160 bits de decoded[] dans les 20 octets du fichier .key
static void encode_key(const unsigned short decoded[10], uint8_t *k)
{
	for (int i = 0; i < 20; i++) k[i] = 0;
	for (int b = 0; b < 10 * 16; b++)
	{
		int bit = (317 - b) % 160;
		if (decoded[b / 16] & (0x8000 >> (b % 16)))
			k[bit / 8] |= 1 << ((bit ^ 7) % 8);
	}
}

static uint32_t crc32(const std::vector<uint16_t> &v)
{
	// CRC-32 (IEEE) des mots en gros-boutiste (octet fort d'abord), comme cps_common::crc::crc32 sur les octets
	uint32_t c = 0xffffffff;
	for (uint16_t w : v)
		for (int s : { 8, 0 }) {
			c ^= (w >> s) & 0xff;
			for (int i = 0; i < 8; i++) c = (c >> 1) ^ (0xedb88320 & (0 - (c & 1)));
		}
	return ~c;
}

int main()
{
	const int length = 0x400000; // CODE_SIZE
	std::vector<uint16_t> rom(length / 2), dec(length / 2);
	for (uint32_t a = 0; a < rom.size(); a++) rom[a] = uint16_t(a * 0x9e37 + 0x1234); // motif de la carte T801
	running_machine m;
	uint32_t seed = 0x2468ace1;
	printf("# key(20 octets) key0 key1 lower upper crc32(dec) puis 16 lignes « adresse_mot valeur »\n");
	for (int t = 0; t < 5; t++)
	{
		uint8_t k[20];
		for (int i = 0; i < 20; i++) { seed = seed * 1103515245 + 12345; k[i] = t == 0 ? 0xff : uint8_t(seed >> 16); }
		if (t == 4)
		{
			// limite haute dans la ROM : ~decoded[9] & 0x3ff = 0x3f -> upper = 0x100000
			const unsigned short d[10] = { 0x1357, 0x9bdf, 0x2468, 0xace0, 0, 0, 0, 0x4000, 0x0900, uint16_t(~0x3f) };
			encode_key(d, k);
		}
		uint32_t key[2], lower, upper;
		decode_key(k, key, lower, upper);
		cps2_decrypt(m, rom.data(), dec.data(), length, key, lower / 2, upper / 2);
		printf("key ");
		for (int i = 0; i < 20; i++) printf("%02x", k[i]);
		printf(" %08x %08x %06x %06x %08x\n", key[0], key[1], lower, upper, crc32(dec));
		const uint32_t probes[16] = { 0, 1, 2, 0x3e, 0x200, 0x1234, 0xffff, 0x10000, 0x10001, 0x7ffff, 0x80000, 0xfffff, 0x100000, 0x123456 & 0x1fffff, 0x1ffffe, 0x1fffff };
		for (uint32_t p : probes) printf("%06x %04x\n", p, dec[p]);
	}
	return 0;
}
