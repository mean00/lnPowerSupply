#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
use cty::c_char;
use rnarduino as rn;

const ADC_SAMPLE : usize = 8;

/**
 * \brief runTime
 */
struct runTime
{
   eventGroup : rn::lnFastEventGroup,
   adc        : rn::lnTimingAdc,
   output     : [u16; ADC_SAMPLE*2],
   pins       : [rn::lnPin; 2] ,   
}
/**
 * 
 * 
 */
impl runTime
{
   /**
    *     
    */
   fn new() -> runTime
   {
      unsafe{
      runTime
      {
         eventGroup  :  rn::lnFastEventGroup::new(),
         adc         :  rn::lnTimingAdc::new(0),
         output      :  [0,0,0,0, 0,0,0,0, 0,0,0,0 ,0,0,0,0],
         pins        :  [rn::PA0 as rn::lnPin,rn::PA1  as rn::lnPin] , // PA0 + PA1
      }
      }
   }
   /**
    * 
    */
   fn run(&mut self) -> ()
   {
      unsafe{
      self.eventGroup.takeOwnership();   
      self.adc.setSource(3,3,1000,2,self.pins.as_ptr() );
      }
      
      loop
      {   
         let mut ev : u32 = 0;
         unsafe{
         ev = self.eventGroup.waitEvents( 0xff , 100);
         }
   
         let mut sbat  : f32=0.;
         let mut maxCurrent : i32=0;
         self.runAdc( &mut sbat, &mut maxCurrent);
   
         Logger("rust:>\n");
      }  
   }
   /**
    * 
    */
   fn runAdc(&mut self, fvbat : &mut f32 ,maxCurrentSlopped :  &mut i32 )
   {
      
      unsafe {
      self.adc.multiRead(ADC_SAMPLE as i32 ,self.output.as_mut_ptr() ); 
      }
       
      let mut max0 : isize =0;
      let mut max1 : isize =0;
      
      for i in 0..ADC_SAMPLE   {
         max0+=self.output[i+i] as isize;   
         max1+=self.output[i+i+1] as isize;
      }
   
      let mut vbat : isize =0;
      let mut maxCurrent : isize = 0;
       
       vbat = max0 + ((ADC_SAMPLE-1)/2) as isize;
       vbat = vbat /(ADC_SAMPLE as isize);
       maxCurrent = (max1 as isize) + (((ADC_SAMPLE as isize)-1)/2);
       maxCurrent = maxCurrent/(ADC_SAMPLE as isize);
   
       *fvbat = vbat as f32;    
       *fvbat=*fvbat*9.;
       *fvbat/=1000.;
       
       maxCurrent=maxCurrent*maxCurrent;
       // 0..4095 -> 0.. 4A amp=val*1.5+50
       // so max=4000/1.5=2660
       maxCurrent/=6300; // 0..4000
       maxCurrent-=maxCurrent&7;
       maxCurrent+=50;
       *maxCurrentSlopped=maxCurrent as i32;
   }
}
/**
 * \fn Logger
 * \brief simple logger version
 * 
 */
fn Logger(st : &str) -> ()
{
   unsafe{
      rn::Logger(st.as_ptr() as *const c_char);
   }
}

/**
 * \fn rnLoop
 * 
 * 
 */
#[no_mangle]
pub extern "C" fn rnLoop() -> ()
{
   let mut r : runTime = runTime::new();
   r.run();
}
  
// EOF