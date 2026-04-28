This folder contains all the code required to reproduce our experiments and regenerate the tables and figures from our paper.

There are two levels of reproducibility:

* Regenerate the plots and tables of the paper from the provided raw data.
* Re-run the experiments and regenerate the plots using newly produced results.

---

# Installation

The only requirements are **Python** and **Rust**. Rust can be installed in a single command by following the instructions at:
[https://rust-lang.org/tools/install/](https://rust-lang.org/tools/install/)

Then run:

```
make install
```

to install all dependencies.

Alternatively, you can use a Docker container:

```
docker build -t artifact-hlut-tches .
docker run -it artifact-hlut-tches bash
```

---

# Quick Reproduction

## Reproducing the Plots

To regenerate all figures from the paper using the provided raw data, run:

```
make plots MODE=paper
```

The generated figures will be located in `figures/paper` and should match those in the paper.

## Full Reproduction

To reproduce all experiments, run:

```
make reproduce
```

This command:

* Runs all benchmarks and experiments to regenerate raw data files (stored in `data/regenerated`)
* Regenerates figures from this new data (stored in `figures/regenerated`)

If you are using Docker and want to retrieve the figures, run:

```
docker cp artifact-hlut-tches/figures/paper ./figures
docker cp artifact-hlut-tches/figures/regenerated ./figures
```

This reproduction can take a while depending of your hardware. On our computing server equipped with an AMD
Ryzen Threadripper PRO 7995WX (96 cores), this takes about one hour.

## Stand-alone Tool

The decomposition generation is not included in the full reproduction script, as it is computationally intensive. Instead, we provide a stand-alone tool to decompose any S-box.

Example:

```
make decompose S=2 P=3 N=8 GAMMA=1.05 SBOX_FILE=aes.sbox
```

This generates a one-bit block decomposition of the AES S-box. The output can be found in `./search`.

**S-box file format:**
Provide output values in order, on a single line, separated by spaces. See `aes.sbox` for an example.

---

# Detailed Documentation

## Decompositions

We provide the decompositions and S-boxes used in our experiments in:

```
decompositions/paper
```

To regenerate decompositions for all sizes:

```
make decompositions
```

⚠️ This is computationally expensive for large sizes. For targeted use, prefer the stand-alone tool.

---

## Plots and Tables

All plots and tables from the paper can be regenerated. Available outputs include:

* **Figure 1** — Timing comparison with classical PBS
* **Figure 5** — Distribution of matrix ranks $\mathcal{A}$ for varying $n$ and $\gamma$
* **Figure 6** — Correlation between Equation 8 margin and full-rank proportion
* **Figure 8** — Exhaustive timing comparison with state-of-the-art methods
* **Figure 9** — Timing results across output sizes
* **Table 1** — Optimal matrix shape parameters
* **Table 2** — Number of PBS operations per configuration
* **Table 3** — Success rates with/without encoding-switching optimization
* **Table 4** — Timings for failure probability $2^{-40}$
* **Table 5** — Timings for failure probability $2^{-128}$

You can generate plots using either:

* `paper` (default): original data
* `regenerated`: newly generated data

Run:

```
MODE=[paper|regenerated] make plots
```

---

## Benchmarks

Four benchmarks are available:

* Our technique
* Tree-Based Method
* WoP-PBS
* Traditional bootstrapping (CJP)

Run individually:

```
make hlut-full
make tbm
make wopppbs
make cjp
```

Or all at once:

```
make bench
```

For our method, you can set the failure probability:

* `PERROR=40` → $p_{fail} = 2^{-40}$ (default)
* `PERROR=128` → $p_{fail} = 2^{-128}$

To reproduce Figure 9 (smaller outputs):

```
make hlut-partial
```

---

## Experiments

The following experiments can be reproduced (and corresponding plots generated with `MODE=regenerated`):

* Figure 5:

  ```
  make experiments-ranks-distribution
  ```
* Figure 6:

  ```
  make experiments-correlation-margin
  ```
* Table 1:

  ```
  make experiments-shapes
  ```
* Table 2:

  ```
  make experiments-counts-pbs
  ```
* Table 3:

  ```
  make experiments-encodings
  ```
