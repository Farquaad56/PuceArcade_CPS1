// Oracle YM2151 : pilote ymfm (BSD-3, ref/ymfm). Génère tests-data/ym2151/*.bin
// Format : 1000 échantillons x 2 voies i16 LE [gauche, droite] = 4000 octets.
// Conventions (identiques à l'adaptateur Rust, cartes T306/T307) :
//  - 1 échantillon toutes les 64 horloges YM (prescale 2 x 32 opérateurs) ; l'échantillon n (n>=1) est produit à ym_time = n*64.
//  - Avant l'échantillon n : écritures planifiées "avant l'échantillon n" (ym_time = (n-1)*64), puis timers dont
//    l'échéance <= n*64 (ordre chronologique, engine_timer_expired), puis generate(1).
//  - set_timer(t, d) : échéance = ym_time + d (d<0 => arrêt). set_busy_end(c) : busy jusqu'à ym_time + c.
// Compilation : g++ -O2 -std=c++14 -Iref/ymfm/src tools/ymfm_opm_golden.cpp ref/ymfm/src/ymfm_opm.cpp -o ymfm_opm_golden
#include "ymfm_opm.h"
#include <cstdio>
#include <vector>
#include <string>
#include <algorithm>
using namespace ymfm;
struct Intf : ymfm_interface {
  uint64_t t=0; int64_t dl[2]={-1,-1}; uint64_t busy=0; bool irq=false;
  void ymfm_set_timer(uint32_t n,int32_t d) override { dl[n]= d<0 ? -1 : (int64_t)(t+d); }
  void ymfm_set_busy_end(uint32_t c) override { busy=t+c; }
  bool ymfm_is_busy() override { return t<busy; }
  void ymfm_update_irq(bool a) override { irq=a; }
  void fire(uint32_t n){ m_engine->engine_timer_expired(n); }
};
struct W { int before; int off; int val; };
static void run(const std::string&name,std::vector<W> w,bool status=false){
  Intf in; ym2151 chip(in); chip.reset();
  std::vector<int16_t> out; std::vector<uint8_t> st; std::vector<uint8_t> irq;
  size_t wi=0;
  for(int n=1;n<=1000;n++){
    in.t=(uint64_t)(n-1)*64;
    while(wi<w.size()&&w[wi].before<=n){ chip.write(w[wi].off,w[wi].val); wi++; }
    uint64_t target=(uint64_t)n*64;
    for(;;){ int b=-1; for(int k=0;k<2;k++) if(in.dl[k]>=0&&(uint64_t)in.dl[k]<=target&&(b<0||in.dl[k]<in.dl[b])) b=k;
      if(b<0)break; in.t=in.dl[b]; in.dl[b]=-1; in.fire(b); }
    in.t=target;
    ym2151::output_data o; chip.generate(&o,1);
    for(int c=0;c<2;c++){ int v=o.data[c]; v=std::max(-32768,std::min(32767,v)); out.push_back((int16_t)v); }
    if(status){ st.push_back(chip.read(1)); irq.push_back(in.irq?1:0); }
  }
  FILE*f=fopen(("tests-data/ym2151/"+name+".bin").c_str(),"wb"); fwrite(out.data(),2,out.size(),f); fclose(f);
  if(status){ f=fopen(("tests-data/ym2151/"+name+"_status.bin").c_str(),"wb"); fwrite(st.data(),1,st.size(),f); fclose(f);
              f=fopen(("tests-data/ym2151/"+name+"_irq.bin").c_str(),"wb"); fwrite(irq.data(),1,irq.size(),f); fclose(f); }
  long nz=0; for(auto v:out) nz+= v!=0; printf("%-10s non-zero=%ld\n",name.c_str(),nz);
}
// r(w, avant, registre, valeur) : écrit l'adresse (offset 0) puis la donnée (offset 1)
static void r(std::vector<W>&w,int b,int reg,int v){ w.push_back({b,0,reg}); w.push_back({b,1,v}); }
// voix simple sur le canal ch : algo, fb, tl par opérateur, kc
static void voice(std::vector<W>&w,int b,int ch,int algo,int fb,int tl,int kc){
  r(w,b,0x20+ch,0xC0|(fb<<3)|algo); r(w,b,0x28+ch,kc); r(w,b,0x30+ch,0x00); r(w,b,0x38+ch,0x00);
  for(int op=0;op<4;op++){ int o=op*8+ch; r(w,b,0x40+o,0x01+op); r(w,b,0x60+o,tl+op*4); r(w,b,0x80+o,0x1F); r(w,b,0xA0+o,0x05); r(w,b,0xC0+o,0x02); r(w,b,0xE0+o,0x24); }
}
int main(){
  run("silence",{});
  { std::vector<W> w; voice(w,1,0,7,0,0x10,0x4A); r(w,2,0x08,0x78|0); run("fm_algo7",w); }
  { std::vector<W> w; voice(w,1,3,4,5,0x08,0x3E); r(w,2,0x08,0x78|3); r(w,600,0x08,0x00|3); run("fm_algo4_keyoff",w); }
  { std::vector<W> w; voice(w,1,1,0,7,0x14,0x5C); r(w,1,0x18,0xC0); r(w,1,0x19,0x7F); r(w,1,0x19,0xFF); r(w,1,0x1B,0x02); r(w,1,0x38+1,0x73); r(w,2,0x08,0x78|1); run("lfo",w); }
  { std::vector<W> w; voice(w,1,7,7,0,0x10,0x40); r(w,1,0x0F,0x80|0x10); r(w,2,0x08,0x78|7); run("noise",w); }
  { std::vector<W> w; r(w,1,0x10,0xFF); r(w,1,0x11,0x00); r(w,1,0x12,0xF0); r(w,1,0x14,0x3F); r(w,400,0x14,0x15); r(w,700,0x14,0x30); run("timers",w,true); }
  { std::vector<W> w; for(int ch=0;ch<8;ch++){ voice(w,1,ch,ch,ch&7,0x18,0x20+ch*9); r(w,2+ch*50,0x08,0x78|ch); } run("mix",w); }
  return 0;
}
