#import "PLYCallback.h"

@implementation PLYCallback {
@private
    void *_data;
    void (*_hook)(void *data, id argument);
    void (*_free)(void *data);
}

- (instancetype)initWithHook:(void (*)(void *, id))hook
                        free:(void (*)(void *))free
                        data:(void *)data {
    if ((self = [super init])) {
        _hook = hook;
        _free = free;
        _data = data;
    }

    return self;
}

- (void)callWithArgument:(id)argument {
    _hook(_data, argument);
}

- (void)dealloc {
    _free(_data);
}

@end