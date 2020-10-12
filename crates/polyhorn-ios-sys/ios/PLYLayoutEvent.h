#import <Foundation/Foundation.h>
#import <CoreGraphics/CoreGraphics.h>

@interface PLYLayoutEvent : NSObject

- (instancetype)initWithFrame:(CGRect)frame;

@property (nonatomic, assign, readonly) CGRect frame;

@end
