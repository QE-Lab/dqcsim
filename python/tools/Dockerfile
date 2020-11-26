ARG MANYLINUX=2014
FROM quay.io/pypa/manylinux${MANYLINUX}_x86_64

ARG PYTHON_VERSION=38
ARG AUDITWHEEL_VERSION=3.2
ENV PYBIN /opt/python/cp${PYTHON_VERSION}-cp${PYTHON_VERSION}*/bin

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y && \
	curl -sSfL https://github.com/swig/swig/archive/rel-4.0.1.zip -o swig.zip && \
	unzip -qq swig.zip && \
	cd swig-rel-4.0.1 && \
	./autogen.sh && \
	./configure --without-pcre && \
	make && \
	make install && \
	${PYBIN}/pip3 install --force-reinstall auditwheel==${AUDITWHEEL_VERSION} && \
	yum install -y cmake3 && ln -s /usr/bin/cmake3 /usr/bin/cmake

ENV PATH=$PATH:/root/.cargo/bin

WORKDIR /io
ENTRYPOINT ["bash", "-c", "${PYBIN}/python3 setup.py bdist_wheel"]
