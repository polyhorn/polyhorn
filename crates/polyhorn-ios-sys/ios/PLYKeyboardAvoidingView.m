#import "PLYKeyboardAvoidingView.h"

@implementation PLYKeyboardAvoidingView

- (id)init {
    if ((self = [super init])) {
        NSNotificationCenter *center = [NSNotificationCenter defaultCenter];
        [center addObserver:self selector:@selector(keyboardWillShow:)
                       name:UIKeyboardWillShowNotification object:nil];
        [center addObserver:self selector:@selector(keyboardDidShow:)
                       name:UIKeyboardDidShowNotification object:nil];
        [center addObserver:self selector:@selector(keyboardWillChangeFrame:)
                       name:UIKeyboardWillChangeFrameNotification object:nil];
        [center addObserver:self selector:@selector(keyboardDidChangeFrame:)
                       name:UIKeyboardDidChangeFrameNotification object:nil];
        [center addObserver:self selector:@selector(keyboardWillHide:)
                       name:UIKeyboardWillHideNotification object:nil];
        [center addObserver:self selector:@selector(keyboardDidHide:)
                       name:UIKeyboardDidHideNotification object:nil];
    }

    return self;
}

- (void)dealloc {
    NSNotificationCenter *center = [NSNotificationCenter defaultCenter];
    [center removeObserver:self name:UIKeyboardWillShowNotification object:nil];
    [center removeObserver:self name:UIKeyboardDidShowNotification object:nil];
    [center removeObserver:self name:UIKeyboardWillChangeFrameNotification object:nil];
    [center removeObserver:self name:UIKeyboardDidChangeFrameNotification object:nil];
    [center removeObserver:self name:UIKeyboardWillHideNotification object:nil];
    [center removeObserver:self name:UIKeyboardDidHideNotification object:nil];
}

- (void)keyboardWillShow:(NSNotification *)notification {
    NSValue *value = [notification.userInfo objectForKey:UIKeyboardFrameEndUserInfoKey];
    CGRect bounds = value.CGRectValue;

    [_onKeyboard callWithArgument:@(bounds.size.height)];

    [self updateLayout];
}

- (void)keyboardDidShow:(NSNotification *)notification {
    // Unimplemented.
}

- (void)keyboardWillChangeFrame:(NSNotification *)notification {
    NSValue *value = [notification.userInfo objectForKey:UIKeyboardFrameEndUserInfoKey];
    CGRect bounds = value.CGRectValue;

    [_onKeyboard callWithArgument:@(bounds.size.height)];

    [self updateLayout];
}

- (void)keyboardDidChangeFrame:(NSNotification *)notification {
    // Unimplemented.
}

- (void)keyboardWillHide:(NSNotification *)notification {
    [_onKeyboard callWithArgument:@(0.0)];

    [self updateLayout];
}

- (void)keyboardDidHide:(NSNotification *)notification {
    // Unimplemented.
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
        [view layoutSubviews];
}

- (void)layoutSubviews {
    [super layoutSubviews];
    
    [self updateLayout];
}

@end
