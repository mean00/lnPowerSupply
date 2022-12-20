
use super::main_loop;
use crate::settings::*;

impl  <'a> main_loop  <'a>
{
    pub fn stop_due_to_low_voltage(&mut self)
    {

    }
    /**
     * 
     */

    pub fn run_adc(&mut self) -> (f32, i32) // vbat, maxCurrent non linear scale
    {
       
       self.adc.multiRead(ADC_SAMPLE as i32 ,self.output.as_mut_ptr() ); 
       let mut max0 : isize =0;
       let mut max1 : isize =0;
       
       for i in 0..ADC_SAMPLE   {
          max0+=self.output[i+i] as isize;   
          max1+=self.output[i+i+1] as isize;
       }
    
       let mut vbat : isize ;
       let mut maxCurrent : isize ;
       
       vbat = max0 + ((ADC_SAMPLE-1)/2) as isize;
       vbat = vbat /(ADC_SAMPLE as isize);
 
       maxCurrent = (max1 as isize) + (((ADC_SAMPLE as isize)-1)/2);
       maxCurrent = maxCurrent/(ADC_SAMPLE as isize);
    
       let mut fvbat = vbat as f32;    
       fvbat=fvbat*9.;
       fvbat/=1000.;
       
       maxCurrent=maxCurrent*maxCurrent;
       // 0..4095 -> 0.. 4A amp=val*1.5+50
       // so max=4000/1.5=2660
       maxCurrent/=6300; // 0..4000
       maxCurrent-=maxCurrent&7;
       maxCurrent+=50;
       let maxCurrentSlopped=maxCurrent as i32;
       (fvbat, maxCurrentSlopped)
    }
}
// EOF