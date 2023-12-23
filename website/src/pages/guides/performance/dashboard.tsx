import React from 'react';
import Layout from '@theme/Layout';
import * as R from 'ramda';

interface MyComponentState {
    data: any;
    summary: any;
}

export default class Dashboard extends React.Component<{}, MyComponentState> {
    async componentDidMount() {
        console.log('Fetch benchmark data');
        const response = await fetch('https://cjycode.com/flutter_rust_bridge/dev/bench/data.js');
        const raw = await response.text();
        const data = JSON.parse(raw.replace('window.BENCHMARK_DATA =', ''));

        // @ts-ignore
        window.BENCHMARK_DATA = data;
        console.log('data', data);

        const benchesMap = collectBenchesPerTestCase(data.entries['Flutter Rust Bridge Benchmark']);
        const benches = R.sortBy((x) => x.name, Array.from(benchesMap, ([name, value]) => ({name, value})));
        console.log(benches);

        let summary = R.map(({name, value}) => {
            const benchValue = R.last(value).bench.value;
            const [stem, platform] = splitBenchName(name);
            return ({stem, platform, benchValue});
        }, benches);
        summary = Object.entries(R.groupBy((x) => x.stem, summary));
        summary = R.map(([stem, items]) => [stem, R.groupBy((item) => item.platform, items)], summary);
        console.log('summary', summary);

        this.setState({data, summary});
    }

    render() {
        const data = this.state?.data;
        if (data == null) {
            return <p>Loading</p>
        }

        const platforms = ['Windows', 'Macos', 'Linux'];

        return (
            <Layout>
                <div style={{margin: '32px'}}>
                    <h2>Data for last commit</h2>
                    <table>
                        <thead>
                        <tr>
                            <th>Name</th>
                            {platforms.map((x) => <td key={x}>{x}</td>)}
                        </tr>
                        </thead>
                        <tbody>
                        {
                            this.state.summary.map(([name, value]) => <tr key={name}>
                                <td key="name">{name}</td>
                                {platforms.map((x) =>
                                    <td key={x}>{value[x] ? value[x][0].benchValue.toFixed(3) : 'no data yet'}</td>)}
                            </tr>)
                        }
                        </tbody>
                    </table>
                </div>
            </Layout>
        );
    }
}

// From original HTML template
function collectBenchesPerTestCase(entries) {
    const map = new Map();
    for (const entry of entries) {
        const {commit, date, tool, benches} = entry;
        for (const bench of benches) {
            const result = {commit, date, tool, bench};
            const arr = map.get(bench.name);
            if (arr === undefined) {
                map.set(bench.name, [result]);
            } else {
                arr.push(result);
            }
        }
    }
    return map;
}

function splitBenchName(name: string) {
    const platform = JSON.parse(name).platform;
    return [name.replace(platform, "-"), platform];
}