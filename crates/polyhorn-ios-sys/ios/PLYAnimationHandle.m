#import "PLYAnimationHandle.h"

@implementation PLYAnimationHandle

- (instancetype)initWithLayer:(CALayer * _Nonnull)layer
                          key:(NSString * _Nonnull)key {
    if ((self = [super init])) {
        _layer = layer;
        _key = key;
    }

    return self;
}

- (void)dealloc {
    [_layer removeAnimationForKey:_key];
}

@end
