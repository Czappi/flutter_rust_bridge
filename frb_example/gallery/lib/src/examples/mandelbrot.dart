import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/ignore_me/mandelbrot_related.dart';
import 'package:frb_example_gallery/src/rust/api/mandelbrot.dart';

class MandelbrotPageBody extends StatefulWidget {
  const MandelbrotPageBody({super.key});

  @override
  State<MandelbrotPageBody> createState() => _MandelbrotPageBodyState();
}

class _MandelbrotPageBodyState extends State<MandelbrotPageBody> {
  Uint8List? image;
  Duration? computeTime;
  SimpleRunner? runner;
  var numThreads = 1;

  @override
  void dispose() {
    stop();
    super.dispose();
  }

  void start() {
    stop();
    runner =
        SimpleRunner(minDuration: const Duration(milliseconds: 33), () async {
      final watch = Stopwatch()..start();
      final receivedImage = await drawMandelbrot(
        imageSize: const Size(width: 200, height: 200),
        zoomPoint: examplePoint,
        scale: generateScale(),
        numThreads: numThreads,
      );
      watch.stop();

      if (mounted) {
        setState(() {
          image = receivedImage;
          computeTime = watch.elapsed;
        });
      }
    });
  }

  void stop() {
    runner?.dispose();
    runner = null;
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Column(
          children: [
            for (final candidateNumThreads in [1, 2, 4])
              TextButton(
                onPressed: () {
                  numThreads = candidateNumThreads;
                  start();
                },
                child: Text('Start ($candidateNumThreads threads)'),
              ),
            TextButton(onPressed: stop, child: const Text('Stop')),
          ],
        ),
        if (computeTime != null)
          SizedBox(
            width: 128,
            child: Text('Compute time: ${computeTime!.inMilliseconds}ms'),
          ),
        image != null
            ? SizedBox(
                width: 200,
                height: 200,
                child: AnimatedReplaceableImage(
                  image: MemoryImage(image!),
                ),
              )
            : Container(),
      ],
    );
  }
}
