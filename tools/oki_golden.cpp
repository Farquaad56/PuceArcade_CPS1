// Oracle OKI MSM6295 : okiadpcm.cpp de MAME (BSD-3, ref/mame-sound) compilé tel quel avec tools/oki_shim/emu.h ;
// la logique des voix/commandes recopie okim6295.cpp (write, read, generate_adpcm) l.212-361.
// Génère tests-data/oki/<scénario>.bin : 2000 échantillons i32 LE = somme sur les 4 voix de clock(nibble) * VOL[n]
// (VOL = numérateurs de s_volume_table, dénominateur 0x20 ; la valeur flottante MAME = somme / (2048*32)).
// et <scénario>_status.bin : 2000 octets = read() après chaque échantillon.
// Convention : avant l'échantillon n (n>=1, 0-based k=n-1) on applique les commandes planifiées "avant n", puis on génère.
// ROM : rom(a) = (h ^ (h>>15)) & 0xFF, h = a*0x9E3779B1 (u32), SAUF la table des échantillons (a < 0x400) donnée par phrase().
// Compilation : g++ -O2 -std=c++14 -Itools/oki_shim -Iref/mame-sound tools/oki_golden.cpp ref/mame-sound/okiadpcm.cpp -o oki_golden
#include "emu.h"
#include "okiadpcm.h"
#include <cstdio>
#include <vector>
#include <string>
static const int VOL[16]={0x20,0x16,0x10,0x0b,0x08,0x06,0x04,0x03,0x02,0,0,0,0,0,0,0};
// Échantillon s (1..127) : start = 0x400 + s*0x100, stop = start + 0x7F + s ; s=5 invalide (start >= stop)
static uint32_t start_of(int s){ return 0x400 + s*0x100; }
static uint32_t stop_of(int s){ return s==5 ? start_of(s) : start_of(s) + 0x7F + s; }
static uint8_t rom(uint32_t a){
  a &= 0x3ffff;
  if(a < 0x400){ int s=a/8, k=a%8; uint32_t st=start_of(s), sp=stop_of(s);
    switch(k){ case 0: return st>>16; case 1: return st>>8; case 2: return st; case 3: return sp>>16; case 4: return sp>>8; case 5: return sp; default: return 0; } }
  uint32_t h=a*0x9E3779B1u; h^=h>>15; return (uint8_t)(h&0xFF); }
struct Voice { bool playing=false; uint32_t base=0, sample=0, count=0; int vol=0; oki_adpcm_state adpcm; };
struct Oki {
  Voice v[4]; int command=-1;
  uint8_t read(){ uint8_t r=0xf0; for(int i=0;i<4;i++) if(v[i].playing) r|=1<<i; return r; }
  void write(uint8_t c){
    if(command!=-1){ int mask=c>>4;
      for(int i=0;i<4;i++,mask>>=1) if(mask&1){ Voice&vo=v[i]; if(!vo.playing){
        uint32_t b=command*8; uint32_t st=(rom(b)<<16)|(rom(b+1)<<8)|rom(b+2); st&=0x3ffff;
        uint32_t sp=(rom(b+3)<<16)|(rom(b+4)<<8)|rom(b+5); sp&=0x3ffff;
        if(st<sp){ vo.playing=true; vo.base=st; vo.sample=0; vo.count=2*(sp-st+1); vo.adpcm.reset(); vo.vol=VOL[c&15]; } } }
      command=-1; }
    else if(c&0x80) command=c&0x7f;
    else { int mask=c>>3; for(int i=0;i<4;i++,mask>>=1) if(mask&1) v[i].playing=false; }
  }
  int32_t gen(){ int32_t s=0; for(auto&vo:v){ if(!vo.playing) continue;
      int nib=rom(vo.base+vo.sample/2)>>(((vo.sample&1)<<2)^4);
      s+= (int32_t)vo.adpcm.clock(nib)*vo.vol;
      if(++vo.sample>=vo.count) vo.playing=false; } return s; }
};
struct C { int before; uint8_t val; };
static void run(const std::string&n,std::vector<C> cmds){
  Oki o; std::vector<int32_t> out; std::vector<uint8_t> st; size_t ci=0;
  for(int k=1;k<=2000;k++){ while(ci<cmds.size()&&cmds[ci].before<=k){ o.write(cmds[ci].val); ci++; }
    out.push_back(o.gen()); st.push_back(o.read()); }
  FILE*f=fopen(("tests-data/oki/"+n+".bin").c_str(),"wb"); fwrite(out.data(),4,out.size(),f); fclose(f);
  f=fopen(("tests-data/oki/"+n+"_status.bin").c_str(),"wb"); fwrite(st.data(),1,st.size(),f); fclose(f);
  long nz=0; for(auto x:out) nz+=x!=0; printf("%-10s non-zero=%ld\n",n.c_str(),nz);
}
int main(){
  run("silence",{});
  run("voice1",{{1,0x81},{1,0x10}});                                   // échantillon 1, voix 0, volume 0 dB
  run("two_voices",{{1,0x82},{1,0x22},{50,0x83},{50,0x44}});          // éch. 2 voix 1 vol 2 ; éch. 3 voix 2 vol 4
  run("stop",{{1,0x84},{1,0x80},{100,0x08}});                          // éch. 4 voix 3 vol 0 ; arrêt voix 3 (0x08 = bit 3 -> voix 0 !)
  run("busy_ignore",{{1,0x81},{1,0x10},{10,0x82},{10,0x10}});         // 2e départ sur voix 0 occupée : ignoré
  run("invalid",{{1,0x85},{1,0x10}});                                  // éch. 5 : start == stop -> rien
  run("all4",{{1,0x81},{1,0x10},{1,0x86},{1,0x21},{1,0x87},{1,0x43},{1,0x88},{1,0x88},{300,0x78}});
  return 0;
}
