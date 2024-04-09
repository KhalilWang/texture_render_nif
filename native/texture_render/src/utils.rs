 
pub fn cmp_max(num_1:f32,num_2:f32)->f32
 {
     if num_1>num_2 {
         return num_1;
     }
     else 
     {
         return num_2;
     }
 }
 
 pub fn cmp_min(num_1:f32,num_2:f32)->f32
 {
     if num_1>num_2 {
         return num_2;
     }
     else 
     {
         return num_1;
     }
 }

 
 pub fn rgb_to_hsv(color_r:u8,color_g:u8,color_b:u8)->(f32,f32,f32)
 {
     
 
     let r= color_r as f32 /255.0 ;
     let g= color_g as f32 /255.0 ;
     let b= color_b as f32 /255.0 ;
 
     let max = cmp_max(r, cmp_max(g,b)); // 计算最大亮度值
     let min = cmp_min(r, cmp_min(g,b)); // 计算最小亮度值
  
     let mut hue:f32=0.0;
     if max == min {
         hue = 0.0; // 当最大和最小亮度值都等于时，表示该像素处于灰度状态，色调设置为0
     } else if max == r && g >= b {
         hue = 60.0 * ((g - b) / (max - min)) ; // 根据角度公式计算色调值
     } else if max == r && g < b {
         hue = 60.0 * ((g - b) / (max - min)) + 360.0 ; // 根据角度公式计算色调值
     } else if max == g {
         hue = 60.0 * ((b - r) / (max - min)) + 120.0 ; // 根据角度公式计算色调值
     } else if max == b {
         hue = 60.0 * ((r - g) / (max - min)) + 240.0 ; // 根据角度公式计算色调值
     } 
 
     let mut s=max;
     if max>0.0
     {
         s=(max-min)/max;
     }
     let v=max;
    
     return (hue/360.0,s,v);
        
 }

 pub fn hsv_to_rgb(hue:f32,s:f32,v:f32)->(u8,u8,u8)
{
    //println!("hue={},s={},v={}",hue,s,v);
    let h=hue*360.0;
    let i=((h/60.0)%6.0).round() as u32;
    let f=(h/60.0)-(i as f32);
    let p=v*(1.0-s);
    let q=v*(1.0-f*s);
    let t=v*(1.0-(1.0-f)*s);
    let mut r:f32=0.0;
    let mut g:f32=0.0;
    let mut b:f32=0.0; 
    if i==0
        {
            r=v; g=t; b=p;
        }
    else if i==1 {
        r=q;g=v;b=p;
    }
    else if i==2 {
        r=p;g=v;b=t;
    }
    else if i==3 {
        r=p;g=q;b=v;
    }
    else if i==4 {
        r=t;g=p;b=v;
    }
    else if i==5 {
        r=v;g=p;b=q;
    }

    return ((r*255.0).round() as u8,(g*255.0).round() as u8 ,(b*255.0).round() as u8)

}

pub fn set_hue(color_r:u8,color_g:u8,color_b:u8,add_hvalue:f32)->(u8,u8,u8)
{
    let mut hsv=rgb_to_hsv(color_r,color_g,color_b);
    hsv.0=hsv.0+add_hvalue;
    return hsv_to_rgb(hsv.0, hsv.1,hsv.2);
}


pub fn rgb_to_01(rgb:[u8;3])->[f32;3]
{
    return [
        rgb[0] as f32/255.0,
        rgb[1] as f32/255.0,
        rgb[2] as f32/255.0,
    ];
}
pub fn rgba_to_01(rgba:[u8;4])->[f32;4]
{
    return [
        rgba[0] as f32/255.0,
        rgba[1] as f32/255.0,
        rgba[2] as f32/255.0,
        rgba[3] as f32/255.0,
    ];
}

pub fn rgb_to_255(rgb:[f32;3])->[u8;3]
{
    return [
        (rgb[0] *255.0).round() as u8,
        (rgb[1] *255.0).round() as u8,
        (rgb[2] *255.0).round() as u8,
    ];
}
pub fn rgba_to_255(rgba:[f32;4])->[u8;4]
{
    return [
        (rgba[0] *255.0).round() as u8,
        (rgba[1] *255.0).round() as u8,
        (rgba[2] *255.0).round() as u8,
        (rgba[3] *255.0).round() as u8, 
    ];
}

pub fn lerp_rgba_255(rgba_1:[u8;4],rgba_2:[u8;4],t:u8)->[u8;4]
{
    let mut valu1:[f32;4]=rgba_to_01(rgba_1);
    let mut valu2:[f32;4]=rgba_to_01(rgba_2);
    let f_32=t as f32/255.0;
    let result=lerp_arr4(valu1,valu2,f_32);
    return rgba_to_255(result);
}

//数组
pub fn lerp_rgb_arr_255(rgb_1:[u8;3],rgb_2:[u8;3],t:u8)->[u8;3]
{
    let mut valu1:[f32;3]=rgb_to_01(rgb_1);
    let mut valu2:[f32;3]=rgb_to_01(rgb_2);
    let f_32=t as f32/255.0;
    let result=lerp_arr3(valu1,valu2,f_32);
    return rgb_to_255(result);
}

//元类型
pub fn lerp_rgb_tuple_255(rgb_1:(u8,u8,u8),rgb_2:(u8,u8,u8),t:u8)->(u8,u8,u8)
{
    let mut valu1:[f32;3]=rgb_to_01([rgb_1.0,rgb_1.1,rgb_1.2]);
    let mut valu2:[f32;3]=rgb_to_01([rgb_2.0,rgb_2.1,rgb_2.2]);
    let f_32=t as f32/255.0;
    let result=lerp_arr3(valu1,valu2,f_32);
    let result_255= rgb_to_255(result);
    return (result_255[0],result_255[1],result_255[2]);
}

pub fn clamp(value:f32,min:f32,max:f32)->f32{
    if value < min {
        return min;
    } else if value > max {
        return max;
    } else {
        return value;
    }
}

//将值限定在0~1之间
pub fn saturate(value:f32)->f32{
     
    return clamp(value,0.0,1.0);
}

pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

pub fn smoothstep(edge0:f32,edge1:f32,x:f32)->f32
{
    let t=clamp((x-edge0)/(edge1-edge0),0.0,1.0);
    return t*t*(3.0-2.0*t);
}

//lerp 一个float4
pub fn lerp_arr4(arr1:[f32;4],arr2:[f32;4],t:f32)->[f32;4]
{
    return [
        lerp(arr1[0],arr2[0],t),
        lerp(arr1[1],arr2[1],t),
        lerp(arr1[2],arr2[2],t),
        lerp(arr1[3],arr2[3],t),
    ];
}
pub fn lerp_arr3(arr1:[f32;3],arr2:[f32;3],t:f32)->[f32;3]
{
    return [
        lerp(arr1[0],arr2[0],t),
        lerp(arr1[1],arr2[1],t),
        lerp(arr1[2],arr2[2],t),
    ];
}

pub fn mul_arr4(arr1:[f32;4],arr2:[f32;4])->[f32;4]
{
    return [
        arr1[0]*arr2[0],
        arr1[1]*arr2[1],
        arr1[2]*arr2[2],
        arr1[3]*arr2[3],
    ];
}
