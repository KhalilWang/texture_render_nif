#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut,unused_assignments,non_snake_case))]
pub struct UniformRandom
{
    NTAB:i32,
    IA:i32,
    IM:i32,
    IQ:i32,
    IR:i32,
    NDIV:i32,
    MAX_RANDOM_RANGE:u64,
    AM:f32,
    EPS:f32,
    RNMX:f32,
    m_idum:i32,
    m_iy:i32,
    m_iv:[i32;32],
}

impl  UniformRandom
{
   
    pub fn new()-> Self 
    {
        UniformRandom {NTAB:32,
                         IA:16807,
                         IM:2147483647,
                         IQ:127773,
                         IR:2836,
                         NDIV:1+(2147483647-1)/32,
                         MAX_RANDOM_RANGE:0x7FFFFFFF,
                         AM:1.0/2147483647.0,
                         EPS:1.2/1000.0/10000.0,
                         RNMX:1.0-1.2/1000.0/10000.0,
                         m_idum:0,
                         m_iy:0,
                         m_iv:[0;32]}
    }

    pub fn set_seed(&mut self,iseed:i32)
    {
        if iseed<0
        {
            self.m_idum=iseed;
        }
        else
         {   self.m_idum=-iseed;
         }
        
        self.m_iy=0;
    }

    fn generate_random_number(&mut self)->i32
    {
        //let mut m_iv:[i32;32];
        let mut j:i32=self.NTAB+7;
        let mut k:i32=0;
        if self.m_idum<=0 || self.m_iy==0
        {
            if -self.m_idum<1
            {
                self.m_idum=1;
            }
            else {
                self.m_idum=-self.m_idum;
            }
             
            for i in 0..self.NTAB+7+1
            {
                j=self.NTAB+7-i;
                k=self.m_idum/self.IQ;
                self.m_idum=self.IA*(self.m_idum-k*self.IQ) - self.IR*k;
                if self.m_idum<0
                {
                    self.m_idum+=self.IM;
                }
                if j<self.NTAB
                {
                    self.m_iv[j as usize]=self.m_idum;
                }
            }
            self.m_iy=self.m_iv[0];    
        }
        k=self.m_idum/self.IQ;
        self.m_idum=self.IA*(self.m_idum-k*self.IQ)-self.IR*k;
        if self.m_idum<0
        {
            self.m_idum+=self.IM;
        }
        j=self.m_iy/self.NDIV;
        if j>=self.NTAB || j<0
        {
            j&=self.NTAB-1;
        }
        self.m_iy=self.m_iv[j as usize];
        self.m_iv[j as usize]=self.m_idum;
        return self.m_iy;

    }

    pub fn random_float(&mut self,f1_low:f32,f1_high:f32)->f32
    {
        let mut f1=self.AM*self.generate_random_number() as f32;
        if f1>self.RNMX
        {
            f1=self.RNMX;
        }
        return f1*(f1_high-f1_low) + f1_low;
    }

}