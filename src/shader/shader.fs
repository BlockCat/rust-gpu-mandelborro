 #version 140

in vec3 vvv;
out vec4 color;
uniform mat3 transform;
uniform float zoom;
uniform vec3 COLORS[8] = {
    vec3(0.2078, 0.2078, 0.7333),
    vec3(0.3255, 0.0941, 0.4784),
    vec3(0.7333, 0.6667, 0.0863),
    vec3(0.0041, 0.3451, 0.1294),
    vec3(0.3941, 0.3451, 0.5294),
    vec3(0.1941, 0.3451, 0.1294),
    vec3(0.2941, 0.4451, 0.4294),
    vec3(0.3941, 0.6451, 0.1294),
};

/*
void main() {
    float x0 = (transform * vvv).x;
    float y0 = (transform * vvv).y;

    int iteration = 0;
    int max_iteration = 1000;

    float x = 0.0;
    float y = 0.0;
    
    int max_distance = 4;

    float x14 = x - 0.25;

    float q = x14 * x14 + y * y;

    if (q * (q + x14) < 0.25 * y * y) {
        while(x * x + y * y < max_distance && iteration < max_iteration) {
            float xt = x * x - y*y + x0;
            y = 2*x*y + y0;
            x = xt;
            iteration = iteration + 1;
        }
    }

    int c= iteration % 2;
    color = vec4(c, c, c, 1.0);
}
*/

void main() {
    float x0 = (transform * vvv).x;
    float y0 = (transform * vvv).y;

    float iteration = 0;
    int max_iteration = 50000;

    float x = 0.0;
    float y = 0.0;
    int max_distance = (1<<16);
    //int max_distance = 4;
    while(x * x + y * y < max_distance && iteration < max_iteration) {
        float xt = x * x - y*y + x0;
        y = 2*x*y + y0;
        x = xt;
        iteration = iteration + 1;
    }

    if (iteration < max_iteration) {
        float log_zn = log(x*x + y*y) / 2.0;
        float log_2 = 0.69314718056;
        float nu = log(log_zn / log_2) / log_2;
        iteration = iteration + 1 - nu;
    }
    
    vec3 c1 = COLORS[int(iteration) % 8];
    vec3 c2 = COLORS[int(iteration + 1.0) % 8];
    
    float r = log(iteration) / 3;
    float c = 3.32192809;
    float b = 0.78298596083;
    float a = 0.41366809925;
    vec4 color1 = vec4((1 - cos(a * r))/ 2, 1 - cos(b * r) / 2, 1 - cos(c * r) / 2, 1.0);
    vec4 color2 = vec4(mix(c1, c2, iteration - int(iteration)), 1.0);

    color = mix(color1, color2, 0.5);
}