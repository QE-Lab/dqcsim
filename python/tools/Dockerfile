ARG MANYLINUX=1
FROM quay.io/pypa/manylinux${MANYLINUX}_x86_64 as dqcsim-python-base

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
	curl -sSfL https://github.com/swig/swig/archive/rel-3.0.12.zip -o swig.zip && \
	unzip -qq swig.zip && \
	cd swig-rel-3.0.12 && \
	./autogen.sh && \
	./configure --without-pcre && \
	make && \
	make install && \
	for PIP in /opt/python/cp3{5,6,7}*/bin/pip3; do $PIP install auditwheel; done

ENV PATH=$PATH:/root/.cargo/bin

FROM dqcsim-python-base

WORKDIR /io
ENTRYPOINT ["bash", "-c", "for PYTHON in /opt/python/cp3{5,6,7}*/bin/python3; do $PYTHON setup.py bdist_wheel; done; rm -rf target/release"]
