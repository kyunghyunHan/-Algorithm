


pub struct Point {
    x: i64,
    y: i64,
    a: i64,
    b: i64,
}

impl Point {
    pub fn new(&mut self,x:i64, y:i64, a: i64, b: i64) -> Result<Point, String> {
       self.a= a;
       self.b= b;
       self.x= x;
       self.b= b;


       if self.y.pow(2) !=self.x.pow(3)+a*x+b{
        return Err(format!("({}, {}) is not on the curve", x, y));

       }
       Ok(Point { x,y,a,b })
       
    }

}
pub fn main(){
    /*
    타원 곡선은 다음과 같은 식으로 나타냅니다

    y^2 = x^3+ax+b
    
    1차방정식에서 
    y= mx+b라는 방정식이 있습니다.
    여기서 m은 기울기 이고 b는 y의 절편입니다.

   
    (그래프)

    2차방정식은 
    y= ax^2 +bx+c

    (그래프)
    

    3차방정식
    y= ax^3+bx^2+cx+d

    (그래프)
     
    타원곡선도 이와 비슷합니다
        y^2 = x^3+ax+b


     
    타원 곡선과 3차방정식곡선의 기본적인 차이는 왼쪽의 y2 항입니다
    그림과 같이 y2항으로 인해 그래프가 x축에 대칭이 됩니다.

     (그래프)

    타원곡선은 3차곡선보다 기울기가 완만한데 이또한 y2항 떄문입니다.
    계숫값에 따라서 곡선이 분리될수도 있습니다.

     (그래프)
    
    타원곡선은 3차방정식 그래프에서 y>0인 부분에 대해 곡선을 완만하게 하여 그림과 같은 그래프를 만든 후 y>0인 부분에 대해 x축에 대칭이 되도록 만든 그래프라 생각이 가능합니다.


     (그래프)
      (그래프)
       (그래프)

    비트코인에서는 secp2541k이라고 하며다음과 같은 그림이 될수 잇습니다.
    x2= x3+7
    일반정규식은
    y^2= x^3+ax+b
    즉 계숫값이 a= 0,b=7인 곡선으로 정의가 됩니다.

     (그래프)
     */


    

}