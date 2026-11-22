// Oracle YM2151 aléatoire : pilote ymfm (BSD-3, ref/ymfm) avec des écritures pseudo-aléatoires dans tous les registres
// (key-on, mode, bruit, LFO, canaux et opérateurs ; registres de timers exclus : couverts par le scénario « timers »).
// Sortie : tests-data/ym2151/fuzz<g>_w.bin (écritures, 6 octets : échantillon u32 LE, registre, valeur)
//          tests-data/ym2151/fuzz<g>_o.bin (échantillons i32 LE [gauche, droite], AVANT écrêtage et sans conversion i16).
// Conventions : celles de ymfm_opm_golden.cpp (écritures « avant l'échantillon n » à ym_time = (n-1)*64, puis generate(1)).
// Compilation : g++ -O2 -std=c++14 -Iref/ymfm/src tools/ymfm_opm_fuzz.cpp ref/ymfm/src/ymfm_opm.cpp -o ymfm_opm_fuzz
#include "ymfm_opm.h"
#include <cstdio>
#include <vector>
#include <cstdint>
using namespace ymfm;
struct Intf : ymfm_interface {
  uint64_t t=0; uint64_t busy=0;
  void ymfm_set_timer(uint32_t,int32_t) override {}
  void ymfm_set_busy_end(uint32_t c) override { busy=t+c; }
  bool ymfm_is_busy() override { return t<busy; }
  void ymfm_update_irq(bool) override {}
};
// xorshift
static uint32_t s;
static uint32_t rnd(){ s^=s<<13; s^=s>>17; s^=s<<5; return s; }
static void run(int seed,int N){
  s=seed*2654435761u+1;
  char nw[64],no[64]; snprintf(nw,64,"tests-data/ym2151/fuzz%d_w.bin",seed); snprintf(no,64,"tests-data/ym2151/fuzz%d_o.bin",seed);
  FILE*fw=fopen(nw,"wb"); FILE*fo=fopen(no,"wb");
  Intf in; ym2151 chip(in); chip.reset();
  for(int n=1;n<=N;n++){
    in.t=(uint64_t)(n-1)*64;
    int k = (rnd()%100<15) ? 1+rnd()%6 : 0;
    for(int j=0;j<k;j++){
      uint32_t x=rnd(); int reg, val=(rnd()>>8)&0xFF;
      int c=x%20;
      if(c<3) { reg=0x08; }
      else if(c==3){ static const int g[]={0x01,0x0F,0x18,0x19,0x1B}; reg=g[(x>>8)%5]; if(reg==0x01) val&=0x02; }
      else reg=0x20+((x>>8)%0xE0);
      uint8_t rec[6]={(uint8_t)n,(uint8_t)(n>>8),(uint8_t)(n>>16),(uint8_t)(n>>24),(uint8_t)reg,(uint8_t)val};
      fwrite(rec,1,6,fw);
      chip.write(0,reg); chip.write(1,val);
    }
    in.t=(uint64_t)n*64;
    ym2151::output_data o; chip.generate(&o,1);
    int32_t v[2]={o.data[0],o.data[1]}; fwrite(v,4,2,fo);
  }
  fclose(fw); fclose(fo); printf("fuzz%d     %d échantillons\n",seed,N);
}
int main(){ for(int g=1;g<=4;g++) run(g,20000); return 0; }
