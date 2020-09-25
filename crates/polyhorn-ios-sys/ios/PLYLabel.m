#import "PLYLabel.h"

@implementation PLYLabel

- (instancetype)init {
    if ((self = [super init])) {
        self.numberOfLines = 0;
    }

    return self;
}

- (void)updateLayout {
    if (self.layout == nil)
        return;
    
    CGRect oldFrame = self.frame;
    CGRect newFrame = [self.layout fetch];
    
    if (CGRectEqualToRect(oldFrame, newFrame))
        return;
    
    self.frame = newFrame;
    
    for (UIView *view in self.subviews)
        [view setNeedsLayout];
}

- (void)layoutSubviews {
    [super layoutSubviews];
    
    [self updateLayout];
}

@end
