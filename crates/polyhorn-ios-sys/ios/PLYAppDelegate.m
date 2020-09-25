#import "PLYAppDelegate.h"
#import "PLYSpringboard.h"

extern void __polyhorn_main(void);

@implementation PLYAppDelegate

- (void)applicationDidFinishLaunching:(UIApplication *)application {
    __polyhorn_main();
}

@end
