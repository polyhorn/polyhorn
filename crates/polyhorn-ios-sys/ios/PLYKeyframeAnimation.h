#import <Foundation/Foundation.h>
#import <QuartzCore/QuartzCore.h>

@interface PLYKeyframeAnimation : NSObject

- (instancetype)initWithDuration:(CFTimeInterval)duration
                           times:(NSArray<NSNumber *> *)times
                          values:(NSArray *)values;

- (CAKeyframeAnimation *)CAKeyframeAnimationWithKeyPath:(NSString *)keyPath;

@property (nonatomic, assign, readonly) CFTimeInterval duration;
@property (nonatomic, strong, readonly) NSArray<NSNumber *> *times;
@property (nonatomic, strong, readonly) NSArray *values;

@end
