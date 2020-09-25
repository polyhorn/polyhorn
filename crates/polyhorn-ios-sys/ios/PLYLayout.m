#import "PLYLayout.h"

@implementation PLYLayout {
@private
    void *_data;
    CGRect (*_hook)(void *data);
    void (*_free)(void *data);
}

- (instancetype)initWithHook:(CGRect (*)(void *))hook
                        free:(void (*)(void *))free
                        data:(void *)data {
    if ((self = [super init])) {
        _hook = hook;
        _free = free;
        _data = data;
    }

    return self;
}

- (CGRect)fetch {
    return _hook(_data);
}

- (void)dealloc {
    _free(_data);
}

@end