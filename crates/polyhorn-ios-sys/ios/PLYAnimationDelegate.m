#import "PLYAnimationDelegate.h"

@implementation PLYAnimationDelegate

- (instancetype)initWithHandle:(PLYAnimationHandle *)handle {
    if ((self = [super init])) {
        _handle = handle;
    }

    return self;
}

- (void)animationDidStop:(CAAnimation *)anim
                finished:(BOOL)flag {
    // TODO: send the flag to the `onStop` callback.
    if (self.handle)
        [self.handle.onStop callWithArgument:nil];
}

@end