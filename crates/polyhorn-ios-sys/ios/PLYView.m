#import "PLYAnimationDelegate.h"
#import "PLYView.h"

@implementation PLYView

- (instancetype)init {
    if ((self = [super init])) {
        self.opaque = NO;
    }

    return self;
}

- (void)setFrame:(CGRect)frame {
    frame.origin.x += self.layer.transform.m41;

    [super setFrame:frame];
}

- (void)updateLayout {
    for (UIView *view in self.subviews)
        [view setNeedsLayout];
    
    if (self.layout == nil)
        return;
    
    CGRect oldFrame = self.frame;
    CGRect newFrame = [self.layout fetch];
    
    if (CGRectEqualToRect(oldFrame, newFrame))
        return;
    
    self.frame = newFrame;
}

- (void)layoutSubviews {
    [super layoutSubviews];
    
    [self updateLayout];
}

- (void)setOpaqueBackgroundColor:(UIColor *)color {
    _opaqueBackgroundColor = color;

    [self setNeedsDisplay];
}

- (CGPoint)resolvePoint:(PLYPoint)point {
    CGPoint result = CGPointZero;

    if (point.x.kind == PLYDimensionKindPixels)
        result.x = point.x.value;
    else if (point.x.kind == PLYDimensionKindPercentage)
        result.x = point.x.value * self.bounds.size.width;

    if (point.y.kind == PLYDimensionKindPixels)
        result.y = point.y.value;
    else if (point.y.kind == PLYDimensionKindPercentage)
        result.y = point.y.value * self.bounds.size.height;

    return result;
}

- (void)drawRect:(CGRect)dirtyRect {
    const CGFloat kappa = 4 * (sqrt(2.0) - 1.0) / 3.0;

    CGPoint tl = [self resolvePoint:self.cornerRadii.topLeading];
    CGPoint tr = [self resolvePoint:self.cornerRadii.topTrailing];
    CGPoint br = [self resolvePoint:self.cornerRadii.bottomTrailing];
    CGPoint bl = [self resolvePoint:self.cornerRadii.bottomLeading];

    UIBezierPath *path = [UIBezierPath bezierPath];

    // Move to the end of the top-left corner.
    [path moveToPoint:CGPointMake(tl.x, 0.0)];

    // Move to the start of the top-right corner.
    [path addLineToPoint:CGPointMake(self.bounds.size.width - tr.x, 0.0)];

    // Draw the top-right corner.
    [path addCurveToPoint:CGPointMake(self.bounds.size.width, tr.y)
            controlPoint1:CGPointMake(self.bounds.size.width - tr.x + tr.x * kappa, 0.0)
            controlPoint2:CGPointMake(self.bounds.size.width, tr.y - tr.y * kappa)];

    // Move to the start of the bottom-right corner.
    [path addLineToPoint:CGPointMake(self.bounds.size.width, self.bounds.size.height - br.y)];

    // Draw the bottom-right corner.
    [path addCurveToPoint:CGPointMake(self.bounds.size.width - br.x, self.bounds.size.height)
            controlPoint1:CGPointMake(self.bounds.size.width, self.bounds.size.height - br.y + br.y * kappa)
            controlPoint2:CGPointMake(self.bounds.size.width - br.x + br.x * kappa, self.bounds.size.height)];

    // Move to the start of the bottom-left corner.
    [path addLineToPoint:CGPointMake(bl.x, self.bounds.size.height)];

    // Draw the bottom-left corner.
    [path addCurveToPoint:CGPointMake(0.0, self.bounds.size.height - bl.y)
            controlPoint1:CGPointMake(bl.x - bl.x * kappa, self.bounds.size.height)
            controlPoint2:CGPointMake(0.0, self.bounds.size.height - bl.y + bl.y * kappa)];

    // Move to the start of the top-left corner.
    [path addLineToPoint:CGPointMake(0.0, tl.y)];

    // Draw the bottom-left corner.
    [path addCurveToPoint:CGPointMake(tl.x, 0.0)
            controlPoint1:CGPointMake(0.0, tl.y - tl.y * kappa)
            controlPoint2:CGPointMake(tl.x - tl.x * kappa, 0.0)];

    [path closePath];

    [self.opaqueBackgroundColor setFill];
    [path fill];
}

- (void)touchesBegan:(NSSet<UITouch *> *)touches withEvent:(UIEvent *)event {
    [super touchesBegan:touches withEvent:event];

    [self.onPointerDown callWithArgument:nil];
}

- (void)touchesCancelled:(NSSet<UITouch *> *)touches withEvent:(UIEvent *)event {
    [super touchesCancelled:touches withEvent:event];

    [self.onPointerCancel callWithArgument:nil];
}

- (void)touchesEnded:(NSSet<UITouch *> *)touches withEvent:(UIEvent *)event {
    [super touchesEnded:touches withEvent:event];

    [self.onPointerUp callWithArgument:nil];
}

- (PLYAnimationHandle *)addKeyframeAnimation:(PLYKeyframeAnimation *)animation
                                  forKeyPath:(NSString *)keyPath {
    NSString *key = [NSUUID UUID].UUIDString;

    CAAnimation *keyframes = [animation CAKeyframeAnimationWithKeyPath:keyPath];
    PLYAnimationHandle *handle = [[PLYAnimationHandle alloc] initWithLayer:self.layer key:key];
    keyframes.delegate = [[PLYAnimationDelegate alloc] initWithHandle:handle];
    [self.layer addAnimation:keyframes forKey:key];

    return handle;
}

@end
