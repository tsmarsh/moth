Especially git tends to remove empty directories. This leads to moth opreations to fail, when moth is initialised but 'doing', for instance, is missing and a confusing 'please run init' is reported.

Instead, moth store, given moth is initialised, should ensure directories exist before accessing them.
