#import "PLYKeyframeAnimation.h"

@implementation PLYKeyframeAnimation

- (instancetype)initWithDuration:(CFTimeInterval)duration
                           times:(NSArray<NSNumber *> *)times
                          values:(NSArray *)values {
    if ((self = [super init])) {
        _duration = duration;
        _times = times;
        _values = values;
    }

    return self;
}

- (CAKeyframeAnimation *)CAKeyframeAnimationWithKeyPath:(NSString *)keyPath {
    CAKeyframeAnimation *animation = [CAKeyframeAnimation animationWithKeyPath:keyPath];
    animation.duration = self.duration;
    animation.keyTimes = self.times;
    animation.values = self.values;
    return animation;
}

@end
