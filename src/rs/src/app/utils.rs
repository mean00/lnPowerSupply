use super::main_loop;
use crate::settings::*;

impl  <'a> main_loop  <'a>
{
   /**
    * 

    */
    pub fn stop_due_to_low_voltage(&mut self)
    {

    }
    /**
     * Run the ADC to get 
     * vbat as float in volt
     * maxCurrent as usize (0..4095)
     * /!\ max Current is actually maxCurrent*maxCurrent to have more accuracy at low values
     */

    pub fn run_adc(&mut self) -> (f32, usize) 
    {
       
       self.adc.multiRead(ADC_SAMPLE as i32 ,self.output.as_mut_ptr() ); 
       let mut max0 : usize =0;
       let mut max1 : usize =0;
       
       for i in 0..ADC_SAMPLE   {
          max0+=self.output[i+i] as usize;   
          max1+=self.output[i+i+1] as usize;
       }
    
       let mut vbat : usize ;
       let mut maxCurrent : usize ;
       
       vbat = max0 + ((ADC_SAMPLE-1)/2) ;
       vbat = vbat /ADC_SAMPLE ;
 
       maxCurrent = (max1 ) + (((ADC_SAMPLE )-1)/2);
       maxCurrent = maxCurrent/(ADC_SAMPLE );
    
       let mut fvbat = vbat as f32;    
       fvbat=fvbat*9.;
       fvbat/=1000.;
       
       maxCurrent=maxCurrent*maxCurrent;
       // 0..4095 -> 0.. 4A amp=val*1.5+50
       // so max=4000/1.5=2660
       maxCurrent/=6300; // 0..4000
       maxCurrent-=maxCurrent&7;
       maxCurrent+=50;       
       (fvbat, maxCurrent)
    }
}
// EOF