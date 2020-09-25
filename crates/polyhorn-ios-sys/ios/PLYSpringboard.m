#import <Foundation/Foundation.h>

#import "PLYSpringboard.h"

void poly_parachute(void (*fn)(void *), void *data) {
    dispatch_async(dispatch_get_main_queue(), ^{
        fn(data);
    });
}