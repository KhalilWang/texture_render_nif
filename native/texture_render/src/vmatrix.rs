#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
 
const PI:f32=3.14159274;
pub struct VMatrix
{
    matrix:[[f32;4];4] ,
}

 
impl VMatrix
{
    pub fn new()->Self
    {
        VMatrix{matrix:[
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                [0.0,0.0,0.0,0.0],
                        ]}
    }

    pub fn get_row(&self,row:usize)->[f32;4]
    {
        return self.matrix[row];
    }
 
    pub fn get_col(&self,col:usize)->[f32;4]{
        let mut arr:[f32;4]=[0.0,0.0,0.0,0.0];
        for index in 0..arr.len()
        {
            arr[index]=self.matrix[index][col];
        }
        return arr;
    }
    
    //规一化
    pub fn identity(& mut self)
    {
        self.matrix[0][0]=1.0;  self.matrix[0][1]=0.0;  self.matrix[0][2]=0.0;  self.matrix[0][3]=0.0;  
        self.matrix[1][0]=0.0;  self.matrix[1][1]=1.0;  self.matrix[1][2]=0.0;  self.matrix[1][3]=0.0;  
        self.matrix[2][0]=0.0;  self.matrix[2][1]=0.0;  self.matrix[2][2]=1.0;  self.matrix[2][3]=0.0;  
        self.matrix[3][0]=0.0;  self.matrix[3][1]=0.0;  self.matrix[3][2]=0.0;  self.matrix[3][3]=1.0;  
    }
     
    pub fn get_data(&self)->[[f32;4];4]
    {
        return self.matrix;
    }

}
 

 pub fn matrix_build_rotate(angle:f32)->VMatrix
 {
    let mut mat=VMatrix::new();
    let radians=angle*(std::f32::consts::PI/180.0);
    let f_sin=radians.sin();
    let f_cos=radians.cos();
    mat.matrix[0][0]=f_cos;  mat.matrix[0][1]=-f_sin;  mat.matrix[0][2]=0.0;  mat.matrix[0][3]=0.0;
    mat.matrix[1][0]=f_sin;  mat.matrix[1][1]=f_cos;   mat.matrix[1][2]=0.0;  mat.matrix[1][3]=0.0;
    mat.matrix[2][0]=0.0;    mat.matrix[2][1]=-0.0;    mat.matrix[2][2]=1.0;  mat.matrix[2][3]=0.0;
    mat.matrix[3][0]=0.0;    mat.matrix[3][1]=-0.0;    mat.matrix[3][2]=0.0;  mat.matrix[3][3]=1.0;
    return mat;
 }

 pub fn matrix_build_scale(x:f32,y:f32,z:f32)->VMatrix
 {
    let mut mat:VMatrix=VMatrix::new();
    mat.matrix[0][0]=x;    mat.matrix[0][1]=0.0;   mat.matrix[0][2]=0.0;  mat.matrix[0][3]=0.0;
    mat.matrix[1][0]=0.0;  mat.matrix[1][1]=y;     mat.matrix[1][2]=0.0;  mat.matrix[1][3]=0.0;
    mat.matrix[2][0]=0.0;  mat.matrix[2][1]=0.0;   mat.matrix[2][2]=z;    mat.matrix[2][3]=0.0;
    mat.matrix[3][0]=0.0;  mat.matrix[3][1]=-0.0;  mat.matrix[3][2]=0.0;  mat.matrix[3][3]=1.0;
    return mat;
 }

pub fn matrix_build_translation(x:f32,y:f32,z:f32)->VMatrix
{
    let mut mat=VMatrix::new();
    mat.identity();
    mat.matrix[0][3]=x;    
    mat.matrix[1][3]=y;   
    mat.matrix[2][3]=z;  
    return mat;
}

pub fn matrix_multiply(src1:VMatrix,src2:VMatrix)->VMatrix
{
    let mut out_mat:VMatrix=VMatrix::new();
    for row in 0..4 as usize
    {
        for col in 0..4 as usize
        {
            for num in 0..4 as usize
            {
                out_mat.matrix[row][col]=out_mat.matrix[row][col]+src1.matrix[row][num]*src2.matrix[num][col];
            }
        }

    }
    return out_mat;
}

//[SRT]:scale,rotation,translation
pub fn get_srt_matrix(scale:[f32;2],rotation:f32,center:[f32;2],translation:[f32;2])->VMatrix
{
    //矩阵乘顺序  1.TS=T*S ->  2.TSR=R*TS 
    let mat_scale= matrix_build_scale(scale[0],scale[1],1.0);
    let mat_tran= matrix_build_translation(-center[0],-center[1],0.0);
    let t_s=matrix_multiply(mat_tran,mat_scale);
    let mat_rot=matrix_build_rotate(rotation);
    let t_s_r=matrix_multiply(mat_rot,t_s);
    let mat_tran_cneter=matrix_build_translation(center[0]+translation[0],center[1]-translation[1],0.0);
    let t_s_r_tc=matrix_multiply(mat_tran_cneter,t_s_r);
     return t_s_r_tc;
}

pub fn arr2_dot(arr1:[f32;2],arr2:[f32;2])->f32
{
    return arr1[0]*arr2[0]+arr1[1]*arr2[1];
}

pub fn arr3_dot(arr1:[f32;3],arr2:[f32;3])->f32
{
    return arr1[0]*arr2[0]+arr1[1]*arr2[1]+arr1[2]*arr2[2];
}

//pattern_rotation：配置起始结束TextureRotateStart,TextureRotateEnd值然后根据seed数Uniform随机产生
//scale：1目前固定
//offset:配置起始TextureOffsetXStart,TextureOffsetXEnd结束值，然后根据seed数Uniform随机产生
pub fn get_pattern_srt_matrix(pattern_rotation:f32,scale:[f32;2],offset:[f32;2])->VMatrix
{
    let mut m_cur_rotate=pattern_rotation;
    let mut center_offset_y=1220.0* (0.01736*m_cur_rotate+3.155).sin()
                           +677.5*(0.0004791 * m_cur_rotate + 1.965).sin()
                           +851.5*(0.03496*m_cur_rotate + -0.7927).sin();
    center_offset_y/=2409.0;
    let radian=m_cur_rotate*(PI/180.0);
    let mut translation_extra:[f32;2]=[radian.sin()*center_offset_y,-radian.cos()*center_offset_y];
    if m_cur_rotate==0.0
    {
        translation_extra[0]=0.0;
        translation_extra[1]=0.0;
    }
    let mut translation:[f32;2]=[offset[0]+translation_extra[0],offset[1]+translation_extra[1]];
    let center=[0.0,scale[1]];
    return get_srt_matrix(scale,-m_cur_rotate,center,translation);
}


//wear_rot Uniform随机产生值在1.6~1.8之间
pub fn get_wear_srt_matrix(wear_rot:f32,wear_scale:f32,wear_offset:[f32;2])->VMatrix
{
    //patternScale默认1，略
    let mut scale:[f32;2]=[wear_scale,wear_scale];
    return get_srt_matrix(scale,-wear_rot,[0.5,0.5],wear_offset);
}

//grunge_rot Uniform随机产生值在1.6~1.8之间
pub fn get_grunge_srt_matrix(grunge_rot:f32,grunge_scale:f32,grunge_offset:[f32;2])->VMatrix
{
    //patternScale默认1，略
    let mut scale:[f32;2]=[grunge_scale,grunge_scale];
    return get_srt_matrix(scale,-grunge_rot,[0.5,0.5],grunge_offset);
}
 