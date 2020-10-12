#import "PLYLayoutEvent.h"

@implementation PLYLayoutEvent

- (instancetype)initWithFrame:(CGRect)frame {
    if ((self = [super init])) {
        _frame = frame;
    }

    return self;
}

@end
