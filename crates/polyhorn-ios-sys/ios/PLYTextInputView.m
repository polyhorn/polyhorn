#import "PLYTextInputView.h"

@implementation PLYTextInputView {
@private
    UITextField *_textField;
}

- (instancetype)init {
    if ((self = [super init])) {
        _textField = [[UITextField alloc] init];
        _textField.delegate = self;
        [self addSubview:_textField];

        [_textField addTarget:self
                       action:@selector(textDidChange:)
             forControlEvents:UIControlEventEditingChanged];
    }

    return self;
}

- (NSAttributedString *)attributedPlaceholder {
    return _textField.attributedPlaceholder;
}

- (void)setAttributedPlaceholder:(NSAttributedString *)attributedPlaceholder {
    [_textField setAttributedPlaceholder:attributedPlaceholder];
}

- (NSString *)text {
    return _textField.text;
}

- (void)setText:(NSString *)text {
    _textField.text = text;
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

    _textField.frame = self.bounds;
}

- (void)textDidChange:(UITextField *)textField {
    [_onChange callWithArgument:nil];
}

@end