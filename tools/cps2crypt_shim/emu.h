// Shim minimal pour compiler ref/mame-cps2/cps2crypt.cpp hors de MAME (oracle tools/cps2crypt_golden.cpp).
#pragma once
#include <cstdint>
#include <string>
#include <cstdio>
template <typename T> constexpr int BIT(T x, int n) { return (x >> n) & 1; }
template <int B, typename T, typename... U> constexpr T bitswap(T val, U... b) {
	int bits[] = { int(b)... }; T r = 0;
	for (int i = 0; i < B; i++) r |= T(BIT(val, bits[i])) << (B - 1 - i);
	return r;
}
namespace util { template <typename... A> std::string string_format(const char *f, A... a) { char b[64]; std::snprintf(b, sizeof b, f, a...); return b; } }
struct ui_manager { void set_startup_text(const char *, bool) {} };
struct running_machine { ui_manager m_ui; ui_manager &ui() { return m_ui; } };
